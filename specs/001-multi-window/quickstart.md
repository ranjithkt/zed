# Quickstart: Multi-Window Editing (MVP + Enhancements)

This is a developer-facing guide to manually validate the multi-window behavior.

## Prerequisites

- Build and run Zed as you normally do in this repo.
- Open a local project with a populated project tree.

## MVP Manual Test Checklist

### Create an editor-only secondary window

1. In the primary window, **right-click any file/folder in the project tree**.
2. Select **"Open in New Editor Window"** from the context menu.
3. Verify a new secondary window opens showing **only editor tabs/editors** (no project tree, no non-editor panels).

**Alternative**: Use the command palette (Ctrl+Shift+P / Cmd+Shift+P) and search for "new editor window".

### Route project tree opens to the active editor window

1. Focus an editor tab in the **secondary** window.
2. Click a file in the **primary** window’s project tree.
3. Verify the file opens as a new active tab in the **secondary** window.
4. Focus an editor tab in the **primary** window.
5. Click another file in the project tree.
6. Verify it opens in the **primary** window.

### Per-window tab reuse

1. With a file already open in a window, select that same file again in the project tree.
2. Verify the existing tab is activated (no duplicate tab in that window).

### Cross-window sync (content + dirty indicator)

1. Open the same file in both windows.
2. Edit the file in one window.
3. Verify the other window updates to the same content automatically.
4. Verify tab dirty indicators match in both windows (dirty when edited, clean after save).

### Close semantics

1. Close the secondary window:
   - Verify only the secondary window closes; primary stays open.
2. Reopen a secondary window.
3. Close the primary window:
   - Verify all windows for that project close.

---

## User Story 4: Secondary Window Enhancements

### Run/Debug from secondary window

1. Open a file with a `main()` function (e.g., Rust, Python, JavaScript) in a secondary window.
2. Click the **Run** button that appears beside the `main()` function.
3. Verify the program executes (check terminal output in primary window or task output).
4. Repeat with the **Debug** button if applicable.

### Minimal status bar

1. Look at the bottom of the secondary window.
2. Verify you see a **status bar** showing:
   - Current cursor row and column (e.g., "Ln 42, Col 15")
   - An **Outline Panel toggle** button/icon
3. Move the cursor in the editor and verify the row/column updates.
4. Verify the status bar does **NOT** show language, git status, diagnostics, or other primary status bar items.

### Outline Panel toggle

1. Click the **Outline Panel toggle** in the status bar.
2. Verify an **Outline Panel** appears showing document structure (functions, classes, headings).
3. Select a different file tab in the secondary window.
4. Verify the Outline Panel updates to show the structure of the newly selected file.
5. Click the toggle again to hide the Outline Panel.
6. Verify the panel disappears.

### Drag tab to different monitor

*Requires a multi-monitor setup*

1. Open a file in the primary window.
2. Drag the tab header outside the window toward a **different monitor**.
3. Release the drag on the second monitor.
4. Verify a new **secondary editor window** opens on that monitor containing the dragged file.
5. Verify the original tab is removed from the source window (or remains if it was the only copy).


