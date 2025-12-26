# Quickstart: Secondary Window File Menu Integration

Use this guide to manually verify that File menu actions operate on the active editor window (including secondary windows).

## Setup

1. Open a project in Zed (primary window).
2. Create a secondary editor window.
3. Ensure both windows are visible (ideally on different monitors for dialog placement checks).

## Scenario A: Save targets secondary when editing there (P1)

1. In the secondary window, create a new unsaved editor tab (e.g., double-click empty tab strip) and type content.
2. Without clicking inside the primary window’s editor, open **File → Save** from the application menu (in the primary window title bar).
3. Expected:
   - The save flow targets the secondary window’s active editor tab.
   - Any prompts/dialogs (e.g., Save As) appear on the secondary window’s monitor.

## Scenario B: Save targets primary when editing there (P1 regression check)

1. In the primary window, edit a file to create unsaved changes.
2. Open **File → Save**.
3. Expected:
   - The primary window’s active editor tab is saved.

## Scenario C: Save As dialog attaches to correct window (P1)

1. In the secondary window, ensure the active editor is an untitled buffer or a buffer requiring Save As.
2. Trigger **File → Save As…**.
3. Expected:
   - The Save As dialog is modal to the secondary window.

## Scenario D: Close Editor targets active editor window (P2)

1. Open distinct tabs in both primary and secondary windows.
2. Focus the secondary window editor.
3. Trigger **File → Close Editor**.
4. Expected:
   - Only the secondary window’s active tab closes.

## Scenario E: Save All saves across windows (P1 per spec FR-011)

1. Create unsaved changes in files in both primary and secondary windows.
2. Trigger **File → Save All**.
3. Expected:
   - All unsaved files in both windows are saved.

## Notes for platforms

- On Windows/Linux, the File menu UI is in the primary window’s title bar; these tests ensure dispatch targets the correct editor window regardless.
- On macOS, behavior depends on whether the cross-platform menu is enabled; still validate routing when applicable.


