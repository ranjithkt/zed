# Quickstart: Multi-Window Editing (MVP)

This is a developer-facing guide to manually validate the MVP behavior.

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


