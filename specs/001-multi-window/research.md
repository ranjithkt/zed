# Research: Multi-Window Editing (MVP)

## Decisions

### Decision: Share a single `Project` entity across windows in the same project

**Rationale**:

- `project::BufferStore::open_buffer(...)` caches buffers by `ProjectPath`. When two windows share the same `Project` entity, opening the same file returns the same `Buffer` entity, which naturally keeps:
  - text content in sync
  - dirty/unsaved state in sync (tab indicators derive from the shared buffer’s dirty state)

**Alternatives considered**:

- **Separate `Project` per window**: rejected because it would require a bespoke synchronization layer to keep buffers and dirty indicators consistent.

### Decision: Track “active editor window” based on pane focus (not window activation)

**Rationale**:

- `Pane` emits focus events and `Workspace::handle_pane_focused(...)` is invoked when the editor area gains focus.
- Clicking the project tree changes focus to a panel; relying on pane focus prevents project-tree interaction from overwriting the “active editor window” routing target.

**Alternatives considered**:

- **Use OS window activation**: rejected because clicking the project tree in the primary window would make the primary window active even when the user intends opens to go to the secondary editor window.

### Decision: Route project tree opens to the active editor window at the ProjectPanel integration point

**Rationale**:

- `ProjectPanel` already emits `OpenedEntry` events that are handled by the primary window’s `Workspace`, which currently calls `Workspace::open_path_preview(...)` with the primary window context.
- Changing this integration to target a different `WindowHandle<Workspace>` enables routing without rewriting tab/pane logic.

### Decision: Model primary/secondary close semantics as a “project window group”

**Rationale**:

- Secondary close should behave like current `CloseWindow` behavior (remove that window only).
- Primary close must close all secondary windows for that project and avoid losing unsaved work that exists only in secondary windows.
- Coordinating a group close from the primary window provides one place to implement “close secondaries first, then close primary”.

## Constraints / Notes

- MVP avoids introducing new crates; existing collections and `WorkspaceStore` are sufficient to track window-group state.
- Secondary windows should be editor-only in UI chrome. For MVP, the plan focuses on suppressing docks/panels/titlebar content rather than reworking the app menu model (which can be OS-global).

---

## User Story 4: Secondary Window Enhancements (Beyond MVP)

### Decision: Fix Run/Debug actions by providing terminal provider to secondary windows

**Rationale**:

- Run/Debug inline buttons call `Editor::toggle_code_actions` → `CodeActionsMenu` → task scheduling via `Workspace::schedule_resolved_task()`.
- **Root cause identified**: `schedule_resolved_task()` in `crates/workspace/src/tasks.rs` requires `self.terminal_provider` to be set, but secondary windows don't have a `TerminalPanel`, so `terminal_provider` is `None`.
- The `TerminalProvider` is set via `workspace.set_terminal_provider()` in `TerminalPanel::load()` (line 286 in `terminal_panel.rs`).
- **Solution**: Secondary windows should inherit or share the terminal provider from the primary window, or spawn task terminals in the primary window.

**Key code paths discovered**:

1. `Editor::render_run_indicator()` (editor.rs:8780) → creates play button
2. Button click calls `editor.toggle_code_actions(CodeActionSource::RunMenu(...))` (editor.rs:8806)
3. Task selection triggers `tasks_ui::spawn_task_or_modal` → `workspace.schedule_task()` (tasks_ui.rs:98)
4. `Workspace::schedule_resolved_task()` (tasks.rs:49) → `terminal_provider.spawn()` (line 75)
5. **Failure point**: `terminal_provider` is `None` in secondary windows

**Implementation approach**:

- Share the primary window's `TerminalProvider` with secondary windows, OR
- Route task spawning to the primary window via `WorkspaceStore` lookup

### Decision: Minimal status bar shows only essential items

**Rationale**:

- Full status bar includes many items: language selector, diagnostics, git status, copilot, etc.
- For secondary windows, only cursor position (row/column) and Outline Panel toggle are needed.
- Implement as conditional rendering in `StatusBar` based on `WorkspaceWindowRole`.

**Key code paths discovered**:

1. `StatusBar` struct in `status_bar.rs:32` has `left_items` and `right_items` collections
2. Items implement `StatusItemView` trait with `set_active_pane_item()` callback
3. `StatusBar::new()` observes the active pane and updates items when it changes
4. `Workspace::render()` already conditionally renders based on role (see existing secondary window suppression)

**Implementation approach**:

- Add `role: WorkspaceWindowRole` field to `StatusBar` (passed from workspace during construction)
- Create `CursorPositionItem` that implements `StatusItemView` and subscribes to editor cursor changes
- Add `OutlinePanelToggle` button similar to existing panel toggles in `quick_action_bar.rs`
- In `StatusBar::render()`, return minimal UI for `SecondaryEditor` role

**Alternatives considered**:

- **Separate `SecondaryStatusBar` component**: rejected as it duplicates logic; better to conditionally render within existing `StatusBar`.

### Decision: Outline Panel uses right dock slot for secondary windows

**Rationale**:

- `OutlinePanel` is currently designed as a dock panel implementing `Panel` trait.
- For secondary windows, we can selectively enable the right dock for outline panel only.

**Key code paths discovered**:

1. `OutlinePanel` struct in `outline_panel.rs:105` holds:
   - `workspace: WeakEntity<Workspace>` for workspace access
   - `project: Entity<Project>` for file/buffer access
   - `active_item: Option<ActiveItem>` for tracking current editor
2. `OutlinePanel` implements `Panel` trait from `workspace::dock`
3. Toggle is done via `workspace.toggle_panel_focus::<OutlinePanel>()` in `zed.rs:1058`
4. Secondary windows currently suppress docks, but we can selectively enable right dock

**Implementation approach**:

- Modify `Workspace::render()` for secondary windows to show right dock only
- Register `OutlinePanel` in secondary workspace's right dock during creation
- Wire `outline_panel::ToggleFocus` action to secondary window's context
- Reuse existing `OutlinePanel` component unchanged (it already handles active item tracking)

### Decision: Drag-to-monitor uses cursor position at drop time

**Rationale**:

- When a tab is dragged outside the window bounds, the drop position determines target monitor.
- GPUI provides `cx.displays()` to enumerate available displays with bounds.
- Map the drop cursor position to the display containing that point.

**Key code paths discovered**:

1. `DraggedTab` struct in `pane.rs:470` holds dragged tab information
2. `Pane::handle_drag_move()` (pane.rs:3397) receives `DragMoveEvent<T>` with:
   - `event.bounds` - the element bounds
   - `event.event.position` - cursor position during drag
3. Current tab drag-drop is intra-window only (handled by `handle_tab_drop()`)
4. GPUI APIs available in `app.rs`:
   - `cx.displays()` (line 1046) → `Vec<Rc<dyn PlatformDisplay>>`
   - Each `PlatformDisplay` provides `visible_bounds()`, `default_bounds()`, `id()`
5. `Window::display()` (platform.rs:484) → current window's display

**Implementation approach**:

1. Detect when drag ends outside window bounds by tracking drag state
2. On external drop, call `cx.displays()` to get all monitors
3. Find the display whose bounds contain the drop position
4. Call `workspace.open_in_new_editor_window()` with target display ID
5. Extend `open_in_new_editor_window()` to accept optional `DisplayId` for window placement

**Edge cases**:

- Drop between monitors: use display whose center is closest to drop position
- DPI scaling differences: use `display.scale_factor()` when positioning window


