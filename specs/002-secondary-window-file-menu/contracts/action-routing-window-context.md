# Contract: File Menu Action Routing to Active Editor Window

## Goal

Ensure File menu actions operate on the **active editor window** (primary or secondary) for the current project group, even though the application menu UI only exists in the primary window.

## Scope

- Applies to **File menu** actions originating from the client-side `ApplicationMenu` UI (Windows/Linux, and macOS when the cross-platform menu is used).
- Does not change the meaning of actions or their handlers; only changes the dispatch target window.

## Inputs

- **Origin workspace**: the workspace associated with the primary window title bar (the window rendering `ApplicationMenu`)
- **Menu entry**: `OwnedMenu` with `name == "File"`
- **Action**: `Box<dyn gpui::Action>` from `OwnedMenuItem::Action`

## Dispatch Target Selection

For the origin workspace’s project group:

1. Compute `project_key`
2. Resolve `target_window_id = WorkspaceStore::active_editor_window_for_project(project_key)`
3. Resolve `target_workspace_window = WorkspaceStore::workspace_window_for_id(target_window_id)`

### Fallbacks

- If `active_editor_window_for_project` is `None`, dispatch to the current (primary) window.
- If the target window no longer exists, dispatch to the current (primary) window.

## Dispatch Behavior

When the user selects a File menu item:

- Dispatch the action into `target_workspace_window`'s `Window` so that it is delivered to the focused element in that window (typically the active editor).
- **Dialog attachment**: Any dialogs triggered by the action (Save As, Open, close confirmations) should appear attached to the **origin window** (where the File menu lives), NOT the target editor window. This keeps dialogs near the user's mouse/attention.
- Do NOT activate the target window before dispatch; let the action execute silently on the target while dialogs remain on the origin.

## Error Handling

- If resolving the target workspace fails (e.g., weak workspace is gone), log the error and fall back to dispatching in the current window.
- Do not panic; do not silently discard errors.

## Non-goals

- Changing which items appear in the File menu.
- Adding an application menu to secondary windows.
- Altering keyboard shortcut routing (handled by window focus and GPUI dispatch).

## Acceptance Mapping (from spec)

- **FR-001/FR-002/FR-003**: Save/Save As target the focused editor window.
- **FR-004/FR-005**: New/Open target the active editor window rather than always the primary.
- **FR-013**: Dialogs appear attached to the origin window (where the File menu was clicked), not the target editor window.


