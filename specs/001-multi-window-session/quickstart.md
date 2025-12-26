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

## Scenario D: rust-analyzer discovery failure does not spam logs (P2)

1. Open a Rust file in a project state that triggers rust-analyzer “failed to discover workspace”.
2. Quit and relaunch.

**Expected**:
- The error message is not repeated continuously.
- The user receives a single actionable message for the failure.


