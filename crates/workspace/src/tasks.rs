use std::process::ExitStatus;

use anyhow::Result;
use gpui::{AppContext, Context, Entity, Task, WindowHandle};
use language::Buffer;
use project::{TaskSourceKind, WorktreeId};
use remote::ConnectionState;
use task::{DebugScenario, ResolvedTask, SpawnInTerminal, TaskContext, TaskTemplate};
use ui::Window;

use crate::{ProjectKey, Toast, Workspace, WorkspaceWindowRole, notifications::NotificationId};

impl Workspace {
    pub fn schedule_task(
        self: &mut Workspace,
        task_source_kind: TaskSourceKind,
        task_to_resolve: &TaskTemplate,
        task_cx: &TaskContext,
        omit_history: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.project.read(cx).remote_connection_state(cx) {
            None | Some(ConnectionState::Connected) => {}
            Some(
                ConnectionState::Connecting
                | ConnectionState::Disconnected
                | ConnectionState::HeartbeatMissed
                | ConnectionState::Reconnecting,
            ) => {
                log::warn!("Cannot schedule tasks when disconnected from a remote host");
                return;
            }
        }

        if let Some(spawn_in_terminal) =
            task_to_resolve.resolve_task(&task_source_kind.to_id_base(), task_cx)
        {
            self.schedule_resolved_task(
                task_source_kind,
                spawn_in_terminal,
                omit_history,
                window,
                cx,
            );
        }
    }

    pub fn schedule_resolved_task(
        self: &mut Workspace,
        task_source_kind: TaskSourceKind,
        resolved_task: ResolvedTask,
        omit_history: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let spawn_in_terminal = resolved_task.resolved.clone();
        if !omit_history {
            if let Some(debugger_provider) = self.debugger_provider.as_ref() {
                debugger_provider.task_scheduled(cx);
            }

            self.project().update(cx, |project, cx| {
                if let Some(task_inventory) =
                    project.task_store().read(cx).task_inventory().cloned()
                {
                    task_inventory.update(cx, |inventory, _| {
                        inventory.task_scheduled(task_source_kind, resolved_task);
                    })
                }
            });
        }

        log::info!(
            "schedule_resolved_task: terminal_provider={}, role={:?}",
            self.terminal_provider.is_some(),
            self.role
        );
        if let Some(terminal_provider) = self.terminal_provider.as_ref() {
            let task_status = terminal_provider.spawn(spawn_in_terminal, window, cx);

            let task = cx.spawn(async |w, cx| {
                let res = cx.background_spawn(task_status).await;
                match res {
                    Some(Ok(status)) => {
                        if status.success() {
                            log::info!("Task spawn succeeded");
                        } else {
                            log::info!("Task spawn failed, code: {:?}", status.code());
                        }
                    }
                    Some(Err(e)) => {
                        log::error!("Task spawn failed: {e:#}");
                        _ = w.update(cx, |w, cx| {
                            let id = NotificationId::unique::<ResolvedTask>();
                            w.show_toast(Toast::new(id, format!("Task spawn failed: {e}")), cx);
                        })
                    }
                    None => log::info!("Task spawn got cancelled"),
                };
            });
            self.scheduled_tasks.push(task);
        } else if self.role == WorkspaceWindowRole::SecondaryEditor {
            // Secondary windows without a terminal provider should route task
            // spawning to the primary window.
            log::info!("schedule_resolved_task: routing to primary window");
            self.spawn_task_via_primary_window(spawn_in_terminal, cx);
        } else {
            log::warn!(
                "schedule_resolved_task: no terminal provider and not a secondary window (role={:?})",
                self.role
            );
        }
    }

    fn spawn_task_via_primary_window(
        &self,
        spawn_in_terminal: SpawnInTerminal,
        cx: &mut Context<Self>,
    ) {
        let project_key = ProjectKey::for_project(&self.project);
        let workspace_store = self.app_state.workspace_store.clone();

        let primary_window_id = workspace_store
            .read(cx)
            .primary_window_for_project(project_key);

        let primary_workspace: Option<WindowHandle<Workspace>> =
            primary_window_id.and_then(|window_id| {
                workspace_store
                    .read(cx)
                    .workspaces
                    .iter()
                    .find(|handle| handle.window_id() == window_id)
                    .cloned()
            });

        if let Some(primary_workspace) = primary_workspace {
            cx.spawn(async move |_, cx| {
                cx.update(|cx| {
                    primary_workspace
                        .update(cx, |workspace, window, cx| {
                            // The returned Task tracks task execution status, but we
                            // don't need to await it from the secondary window.
                            let _ = workspace.spawn_in_terminal(spawn_in_terminal, window, cx);
                        })
                        .ok();
                })
                .ok();
            })
            .detach();
        } else {
            log::warn!("Cannot spawn task: no primary window found for secondary editor window");
        }
    }

    pub fn start_debug_session(
        &mut self,
        scenario: DebugScenario,
        task_context: TaskContext,
        active_buffer: Option<Entity<Buffer>>,
        worktree_id: Option<WorktreeId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        log::info!(
            "start_debug_session: debugger_provider={}, role={:?}",
            self.debugger_provider.is_some(),
            self.role
        );
        if let Some(provider) = self.debugger_provider.as_mut() {
            provider.start_session(
                scenario,
                task_context,
                active_buffer,
                worktree_id,
                window,
                cx,
            )
        } else if self.role == WorkspaceWindowRole::SecondaryEditor {
            // Secondary windows without a debugger provider should route debug
            // sessions to the primary window.
            log::info!("start_debug_session: routing to primary window");
            self.start_debug_via_primary_window(
                scenario,
                task_context,
                active_buffer,
                worktree_id,
                cx,
            );
        }
    }

    fn start_debug_via_primary_window(
        &self,
        scenario: DebugScenario,
        task_context: TaskContext,
        active_buffer: Option<Entity<Buffer>>,
        worktree_id: Option<WorktreeId>,
        cx: &mut Context<Self>,
    ) {
        let project_key = ProjectKey::for_project(&self.project);
        let workspace_store = self.app_state.workspace_store.clone();

        let primary_window_id = workspace_store
            .read(cx)
            .primary_window_for_project(project_key);

        let primary_workspace: Option<WindowHandle<Workspace>> =
            primary_window_id.and_then(|window_id| {
                workspace_store
                    .read(cx)
                    .workspaces
                    .iter()
                    .find(|handle| handle.window_id() == window_id)
                    .cloned()
            });

        if let Some(primary_workspace) = primary_workspace {
            cx.spawn(async move |_, cx| {
                cx.update(|cx| {
                    primary_workspace
                        .update(cx, |workspace, window, cx| {
                            workspace.start_debug_session(
                                scenario,
                                task_context,
                                active_buffer,
                                worktree_id,
                                window,
                                cx,
                            );
                        })
                        .ok();
                })
                .ok();
            })
            .detach();
        } else {
            log::warn!(
                "Cannot start debug session: no primary window found for secondary editor window"
            );
        }
    }

    pub fn spawn_in_terminal(
        self: &mut Workspace,
        spawn_in_terminal: SpawnInTerminal,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Option<Result<ExitStatus>>> {
        if let Some(terminal_provider) = self.terminal_provider.as_ref() {
            terminal_provider.spawn(spawn_in_terminal, window, cx)
        } else {
            Task::ready(None)
        }
    }
}
