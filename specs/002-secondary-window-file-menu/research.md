# Research: Secondary Window File Menu Integration

**Feature**: `002-secondary-window-file-menu`  
**Date**: 2025-12-26  
**Status**: Complete

## What’s happening today

Secondary editor windows intentionally **do not render the application menu** (File/Edit/…):

- In `crates/title_bar/src/title_bar.rs`, secondary windows call `TitleBar::new_without_menu(...)` and skip building `ApplicationMenu`.
- Therefore, on Windows/Linux (and on macOS when the cross-platform menu is used), the **only place a user can click File menu items** is the primary window’s title bar.

That means **clicking File → Save necessarily originates in the primary window’s menu UI**, even if the user is actively typing in a secondary window on another monitor.

## Root cause of “File menu only affects primary window”

The client-side application menu builds menu entries using `ui::ContextMenu`:

- `crates/title_bar/src/application_menu.rs` builds a `ContextMenu` where each menu item uses `menu.action_checked(name, action, checked)`.
- In `crates/ui/src/components/context_menu.rs`, `action_checked` dispatches by calling:
  - `window.dispatch_action(action, cx)` on the **window that owns the context menu**.

Since that window is the **primary** window (the only one with the menu), File actions are dispatched into the primary window’s element tree, and therefore operate on the primary workspace/editor.

## Existing “active editor window” signal we should reuse

The multi-window MVP already tracks which editor window was last active per project:

- `crates/workspace/src/workspace.rs` updates `WorkspaceStore::set_active_editor_window_for_project(project_key, window_id)` when panes gain focus.
- `WorkspaceStore` exposes:
  - `active_editor_window_for_project(project_key) -> Option<WindowId>`
  - `workspace_window_for_id(window_id) -> Option<WindowHandle<Workspace>>`

This is exactly the “inject active window” hook we need: it survives the fact that the primary window owns the File menu UI.

## Design decision

For **File menu** items (menu name `"File"` from `crates/zed/src/zed/app_menus.rs`), dispatch the action to:

1. The project group’s `active_editor_window` (resolved via `WorkspaceStore`), if present and still alive
2. Otherwise, fall back to dispatching in the current window (primary)

This keeps all existing action handlers and logic intact; only the dispatch target changes.

## Follow-on fixes (still within File menu scope)

After routing is corrected, verify these behaviors against the spec:

- **New File**: ensure it opens in the target (active editor) window, not always a new window.
- **Save All**: spec requires saving across all open windows; likely needs a separate update in workspace logic.

## Testing focus

- File → Save / Save As: must operate on the active editor in the secondary window when the user was working there.
- Multi-monitor: Save dialogs should appear on the monitor of the editor window being targeted.
- Regression: primary-window behavior remains unchanged when primary is the active editor window.

