# Contract: Multi-Window Session Restore

## Goal

Restore multi-window sessions such that each window’s tabs/panes are restored into the **same window** they were previously in, without unintended duplication into the primary window.

## Scope

- Applies to restoring windows/tabs when reopening the app (restore-on-startup behavior).
- Applies to primary and secondary editor windows for the same project roots.
- Includes log/UI suppression for repeated rust-analyzer workspace discovery failures.

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

## Duplication Rules

- A file/tab MUST be restored into a window **only** if it was present in that window’s serialized workspace.
- If a file existed in multiple windows intentionally, it MUST be restored in each of those windows.
- The system MUST NOT create additional duplicates of a file/tab within the primary window unless that duplication existed previously.

## Error Handling

- Missing files: restore proceeds; missing items appear unavailable; other tabs still restore.
- Corrupt workspace state: skip restoring that window; still restore remaining windows; open at least one primary window if all fail.

## rust-analyzer Status Spam Suppression

- Repeated identical rust-analyzer “workspace discovery failed” status updates MUST be deduplicated so logs are not spammed.
- If discovery fails, the user receives a single actionable message per project per launch.


