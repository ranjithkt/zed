# Quickstart: Multi-Window Session Restore

## Scenario A: Restore tabs into correct windows (P1)

1. Open a project in a **primary** window.
2. Open file `A` in the primary window.
3. Open a **secondary editor window** for the same project.
4. Open file `B` in the secondary window.
5. Quit Zed completely.
6. Relaunch Zed with restore-on-startup enabled.

**Expected**:
- Two windows are restored.
- File `A` is open in the primary window.
- File `B` is open in the secondary window.
- File `B` is **not duplicated** into the primary window.

## Scenario A2: System window tabs enabled (macOS)

1. Enable system window tabs (platform/user setting).
2. Create a primary window and at least one secondary editor window with different open files.
3. Quit and relaunch.

**Expected**:
- The app restores using system window tabs (as supported by the platform/setting).
- No unintended duplicate tabs are created for the same file (same file = canonical absolute path within the window’s project origin).

## Scenario B: Intentional duplicates across windows

1. In the primary window, open file `C`.
2. In the secondary window, open file `C` as well.
3. Quit and relaunch.

**Expected**:
- File `C` is restored in **both** windows.
- No additional duplicate tabs are created.

## Scenario C: Closing a secondary window updates restore state (P2)

1. Open primary + secondary window.
2. Close the secondary window.
3. Quit and relaunch.

**Expected**:
- Only the primary window is restored.
- Tabs from the closed secondary window are not restored into the primary.

## Scenario D: rust-analyzer workspace discovery succeeds after restore (P2)

1. Open a Rust project that contains a valid Rust workspace (e.g., a `Cargo.toml` workspace).
2. Open a Rust file so rust-analyzer initializes.
3. Quit and relaunch.

**Expected**:
- rust-analyzer successfully discovers the workspace (no “Failed to discover workspace” for this project).

## Scenario E: Remote restore (WSL/SSH/remote server)

1. Open a remote-backed project (WSL/SSH/remote server) and open at least one file.
2. Open an additional secondary editor window for the same remote project and open a different file.
3. Quit and relaunch.

**Expected**:
- The remote windows restore as remote-backed windows (no mixing local+remote in one window).
- After reconnecting, tabs restore into their original windows without unintended duplicates.

## Scenario F: Remote reconnect failure

1. Ensure a remote-backed project was part of the previous session.
2. Simulate remote reconnect failure (e.g., network down or host unavailable).
3. Relaunch Zed.

**Expected**:
- The remote window still restores in a disconnected state.
- The UI prompts to reconnect.
- Once reconnected, tabs restore into that window.


