# Data Model: Multi-Window Editing (MVP)

This document describes the runtime entities and state needed to implement the MVP multi-window behavior.

## Core Runtime Entities

### `Project` (existing)

- Represents the workspace/project context (worktrees, buffer store, language services, etc.).
- Owns `BufferStore`, which caches `Buffer` entities by `ProjectPath`.

### `Workspace` (existing, extended)

- Represents a single window’s UI + editor state (panes, tabs, docks).
- Extended for MVP with:
  - **`role: WorkspaceWindowRole`**:
    - `Primary` (project tree + app-level chrome)
    - `SecondaryEditor` (editor-only chrome)
  - **`project: Entity<Project>`**:
    - shared across all windows for the same project group

### `WorkspaceStore` (existing, extended)

Currently tracks the set of open `Workspace` windows. Extended for MVP with additional state:

- **Window groups**
  - Key: `ProjectKey` (derived from `Entity<Project>::entity_id()`)
  - Value: `WorkspaceWindowGroup`

- **Active editor window per project**
  - Key: `ProjectKey`
  - Value: `WindowId` of the last window that had editor/pane focus for that project

## Types / State

### `ProjectKey` (new)

- `EntityId` of the `Project` entity for grouping windows at runtime.
- Purpose: reliably associate primary + secondary windows that share the same project.

### `WorkspaceWindowRole` (new)

- `Primary`
- `SecondaryEditor`

### `WorkspaceWindowGroup` (new)

- `project_key: ProjectKey`
- `primary: WindowId`
- `secondaries: HashSet<WindowId>`
- `active_editor_window: WindowId`

## State Transitions

### Creating a secondary editor window

1. Primary workspace triggers “New Editor Window”
2. New GPUI window is created with a `Workspace` root:
   - role = `SecondaryEditor`
   - project = same `Entity<Project>` as primary
3. `WorkspaceStore` registers the new window in the project’s `WorkspaceWindowGroup`

### Updating the active editor window

- When a pane gains focus in any workspace window, update:
  - `active_editor_window` for that project’s group

### Routing project tree opens

- When the project tree requests opening a file:
  - resolve to `ProjectPath`
  - route to `WorkspaceWindowGroup.active_editor_window`
  - in the target window, open/activate the tab

### Closing windows

- **Close secondary**: remove that window; unregister from group.
- **Close primary**:
  - orchestrate closing secondaries (prompt/save as needed)
  - if successful, close primary last and remove the group.


