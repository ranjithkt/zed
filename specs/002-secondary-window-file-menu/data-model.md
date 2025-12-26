# Data Model: Secondary Window File Menu Integration

This feature does not introduce new persisted data. It relies on existing runtime state that already exists to support multi-window editing.

## Core Runtime Entities

### `Workspace` (existing)

- Represents a single window’s editor + UI state (panes, tabs, docks, focused item).
- Has a `role` distinguishing primary vs secondary editor windows.
- Owns/has access to shared `project` state for the window group.

### `WorkspaceStore` (existing)

- Tracks open workspace windows and groups them by project.
- Maintains, per project group, the last editor window that had pane focus.

### `WorkspaceWindowGroup` (existing)

For a given project group:

- `primary`: the primary workspace window id
- `secondaries`: the set of secondary workspace window ids
- `active_editor_window`: the window id that most recently had editor/pane focus

## Key Identifiers

### `ProjectKey` (existing)

- A stable identifier for grouping windows that share the same project.

### `WindowId` (existing)

- Identifies a specific GPUI window.

## State Transitions

### Updating the active editor window

When a pane gains focus within a workspace window:

1. The workspace computes `project_key` for its project group
2. The workspace reports the current `window_id` to the `WorkspaceStore`
3. The `WorkspaceStore` stores it as `active_editor_window` for that `project_key`

### File menu dispatch target selection

When the user triggers a File menu action from the primary window’s application menu:

1. Use the primary window’s workspace to compute the `project_key`
2. Resolve the `active_editor_window` for that project group
3. Dispatch the selected File action into that target window (falling back to primary if missing)

This makes File actions behave as if they were invoked from the editor window the user was actually working in, even though the menu UI lives only in the primary window.


