# Contract: Primary/Secondary Window Close Semantics (MVP)

This contract defines closing behavior for primary and secondary windows that belong to the same project.

## Definitions

- **Primary window**: the window that contains the project tree and app-level UI.
- **Secondary window**: editor-only window associated with the primary window’s project.

## Required Behavior

### Closing a secondary window

- Closing a secondary window MUST close only that window.
- Other windows for the same project MUST remain open.
- If there are unsaved changes in that secondary window, existing save/discard/cancel behavior applies.

### Closing the primary window

- Closing the primary window MUST close all windows for that project (primary + secondaries).
- The close flow MUST not silently discard unsaved changes that exist only in secondary windows.
- If any required confirmation is canceled, the group close MUST be aborted and windows remain open.


