# Contract: Multi-Window Session Restore

## Goal

Restore multi-window sessions such that each window’s tabs/panes are restored into the **same window** they were previously in, without unintended duplication into the primary window.

**Guiding principle (do not reinvent the wheel)**: Within each restored window, behavior MUST be identical to existing single-window restore behavior. This contract only covers the multi-window-specific requirement: restore multiple persisted window snapshots without collapsing/merging them.

## Scope

- Applies to restoring windows/tabs when reopening the app (restore-on-startup behavior).
- Applies to primary and secondary editor windows for the same project roots.
- Applies to local and remote-backed projects (WSL/SSH/remote server), preserving per-window separation by project origin.
- Includes fixing rust-analyzer workspace discovery failures triggered by open/restore flows (no message suppression).
- All within-window behaviors (duplicates, missing files, unsaved buffers, remote reconnect UX) MUST follow the existing single-window restore behavior; this feature only prevents cross-window collapsing/merging.

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

- When platform/user settings enable system window tabs, restoring multiple “windows” may create multiple tabs within a single OS window.
- This feature MUST honor existing platform/setting behavior; it MUST NOT introduce a custom tabbing model.

## Remote reconnect behavior

- If a remote-backed project cannot reconnect during restore, the window MUST still be restored in a disconnected state and prompt the user to reconnect.
- Once reconnected, the window MUST restore the tabs/items that belonged to that window.

## Error Handling

- Missing files: restore proceeds using existing single-window behavior for missing/unavailable items; other tabs still restore.
- Corrupt workspace state: skip restoring that window; still restore remaining windows; open at least one primary window if all fail.

## rust-analyzer Workspace Discovery

- For valid Rust projects (where a Rust workspace exists), rust-analyzer MUST successfully discover the workspace after open/restore.
- The system MUST NOT suppress rust-analyzer status messages as a workaround; the intent is to eliminate the failure condition.


