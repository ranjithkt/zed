# Contract: Multi-Window Session Restore

## Goal

Restore multi-window sessions such that each window’s tabs/panes are restored into the **same window** they were previously in, without unintended duplication into the primary window.

## Scope

- Applies to restoring windows/tabs when reopening the app (restore-on-startup behavior).
- Applies to primary and secondary editor windows for the same project roots.
- Applies to local and remote-backed projects (WSL/SSH/remote server), preserving per-window separation by project origin.
- Includes fixing rust-analyzer workspace discovery failures triggered by open/restore flows (no message suppression).

## Restore Inputs

- **session_id**: The last session identifier.
- **window_stack**: The saved ordering of windows from the last session.
- **serialized_workspaces**: The set of persisted workspaces for the session, each with:
  - `workspace_id`
  - `window_role` (Primary or SecondaryEditor)
  - `paths`, `location`
  - serialized pane tree and items

## Restore Algorithm (High Level)

1. Load the last session’s workspaces as **workspace ids**, ordered by `window_stack` where possible.
2. For each workspace:
   - Open a new window (or attach as a system tab when platform requires).
   - Create a `Workspace` with `database_id = workspace_id`.
   - Load and apply the serialized pane tree + items for that `workspace_id`.
3. Do not merge items across workspaces.

## System window tabs behavior

- When platform/user settings enable system window tabs, restoring multiple windows may create multiple tabs within a single OS window.
- Even in this mode, “window separation” rules still apply: tabs/items from a previously-separate window MUST NOT be duplicated into the primary window beyond what existed in the prior session.

## Remote reconnect behavior

- If a remote-backed project cannot reconnect during restore, the window MUST still be restored in a disconnected state and prompt the user to reconnect.
- Once reconnected, the window MUST restore the tabs/items that belonged to that window.

## Duplication Rules

- A file/tab MUST be restored into a window **only** if it was present in that window’s serialized workspace.
- If a file existed in multiple windows intentionally, it MUST be restored in each of those windows.
- The system MUST NOT create additional duplicates of a file/tab within the primary window unless that duplication existed previously.
- “Same file” is determined by canonical absolute path within a single project origin (local filesystem vs a specific remote environment).

## Error Handling

- Missing files: restore proceeds; missing items appear unavailable; other tabs still restore.
- Corrupt workspace state: skip restoring that window; still restore remaining windows; open at least one primary window if all fail.

## rust-analyzer Workspace Discovery

- For valid Rust projects (where a Rust workspace exists), rust-analyzer MUST successfully discover the workspace after open/restore.
- The system MUST NOT suppress rust-analyzer status messages as a workaround; the intent is to eliminate the failure condition.


