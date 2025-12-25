# Contract: Project Tree → Active Editor Window Open Routing (MVP)

This contract defines how file-open requests originating from the project tree are routed when multiple windows exist for the same project.

## Source

- Project tree interactions in the **primary** window (project panel).

## Routing Rules

- Determine the **active editor window** as the window that most recently had editor/pane focus.
- Project tree file selection opens the file in the active editor window:
  - If already open as a tab in that window → activate the tab.
  - If not open in that window → open a new tab and activate it.
- Opening in one window MUST NOT close the file’s tab in another window.
- Clicking the project tree MUST NOT change which window is considered the active editor window.

## Error Behavior

- If the file cannot be opened, the user must receive an actionable error prompt.
- The prompt should be shown in the primary window (where the interaction originated), even if the open was targeted to a different window.


