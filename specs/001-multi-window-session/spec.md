# Feature Specification: Multi-Window Session Restore

**Feature Branch**: `001-multi-window-session`  
**Created**: December 26, 2025  
**Status**: Draft  
**Input**: User description: "The file now opens in the relevant window which was active. But when the window is getting opened, I see a Rust language server error about failing to discover the workspace. Also, when opening, it restores all tabs from the previous session (including secondary windows) into the primary window, causing duplicates. Closing and opening of windows should support multiple windows and retaining editor tabs per window."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Restore Tabs into Correct Windows (Priority: P1)

A user previously worked with multiple windows (a primary window and one or more secondary editor windows), each with its own set of open editor tabs. After restarting Zed, they expect each window to reopen with the same tabs in the same window, without duplicating tabs into the primary window.

**Why this priority**: This is the core multi-window experience. If session restore collapses secondary-window tabs into the primary window, users lose window organization and must clean up duplicates manually.

**Independent Test**: Can be fully tested by opening two windows, opening different files in each, closing the app, reopening the app, and verifying that each window restores the correct tabs without duplication.

**Acceptance Scenarios**:

1. **Given** a previous session had a primary window with file A open and a secondary window with file B open, **When** the user restarts Zed, **Then** file A opens in the primary window and file B opens in the secondary window (not both in the primary window)
2. **Given** a previous session had file C open in both primary and secondary windows intentionally, **When** the user restarts Zed, **Then** file C is restored in both windows (and only in those windows where it was previously open)
3. **Given** a previous session had a primary window and two secondary windows, each with different active tabs, **When** the user restarts Zed, **Then** all windows are restored and each window’s active tab is restored correctly

---

### User Story 2 - Persist Window/Tab State on Close (Priority: P2)

A user closes one secondary window (or closes the full app) and expects the next launch to restore windows and tabs based on the most recently closed state, rather than restoring a stale layout or merging tabs into the primary window.

**Why this priority**: Correct session restore requires that window and tab state is updated consistently as windows are closed or changed.

**Independent Test**: Can be tested by opening multiple windows, closing a secondary window, restarting Zed, and verifying the closed window stays closed and the remaining windows restore correctly.

**Acceptance Scenarios**:

1. **Given** a user closes one secondary window while leaving others open, **When** they restart Zed, **Then** only the previously open windows are restored (the closed window does not reappear)
2. **Given** a user rearranges which files are open in each window and then closes Zed, **When** they restart Zed, **Then** the restored windows match the most recent state at close time

---

### User Story 3 - Avoid Repeated “Workspace Discovery” Errors on Window Open (Priority: P2)

A user opens or restores a window and expects language features (such as Rust language intelligence) to initialize normally. If a workspace cannot be discovered, the user should not see repeated error spam and should get actionable guidance.

**Why this priority**: Repeated error output reduces trust and makes the product feel unstable, even when editing may otherwise work.

**Independent Test**: Can be tested by restoring a project session and verifying that window open does not produce repeated workspace-discovery errors and that any failure is reported once with actionable instructions.

**Acceptance Scenarios**:

1. **Given** a user opens or restores a project window, **When** language services initialize, **Then** the app does not repeatedly emit the same workspace-discovery error message
2. **Given** workspace discovery fails for a project, **When** the user opens or restores a window, **Then** the user receives a single, clear notification explaining the issue and an action they can take (rather than repeated terminal spam)

---

### Edge Cases

- **Missing files**: If a previously-open file no longer exists, the window still restores and the missing file is shown as unavailable without preventing other tabs from restoring.
- **Unsaved buffers**: If a previous session included unsaved buffers, the system restores them in their original window when safe to do so; otherwise it restores the window without duplicating buffers into other windows.
- **Many windows**: If a session contains many secondary windows, the system restores them without duplicating tabs into the primary window. If a platform-imposed window limit is hit, restore proceeds with as many windows as allowed and reports what could not be restored.
- **Corrupt session state**: If session/window state is unreadable, the system opens a single primary window and does not create duplicate tabs.

### Assumptions & Dependencies

- **Assumptions**:
  - Session restore is enabled and the user is reopening the app rather than opening a project “fresh”.
  - The feature applies to local multi-window projects and should not change the expected behavior of intentionally opening the same file in multiple windows.
- **Dependencies**:
  - Multi-window support already exists (primary + secondary editor windows).
  - Session persistence exists and can represent window state over restarts.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST restore the previous session’s set of windows (primary plus any secondary editor windows) for a project when the user restarts the app
- **FR-002**: System MUST restore each window’s open editor tabs into the same window they were previously associated with
- **FR-003**: System MUST preserve tab order within each restored window
- **FR-004**: System MUST NOT open duplicate editor tabs in the primary window for tabs that belonged to secondary windows in the previous session
- **FR-005**: System MUST only restore a file in multiple windows if that file was open in multiple windows in the previous session
- **FR-006**: System MUST persist window and tab state such that a restart restores the most recent state at close time
- **FR-007**: System MUST restore windows and tabs even if some previously-open files are missing or unavailable
- **FR-008**: System MUST avoid repeatedly emitting the same workspace-discovery error message when opening or restoring windows
- **FR-009**: If workspace discovery fails for a project, System MUST present a single user-visible message with actionable guidance and MUST NOT block window open

### Key Entities

- **Session Window State**: The persisted representation of which windows existed and what tabs were open in each window at the time the session was saved.
- **Window Role**: A window’s type (primary or secondary editor window) used to determine how it is restored.
- **Tab / Editor Item**: An open editor instance in a particular window, including its ordering and active selection.
- **Language Service Initialization**: The startup process that enables language intelligence for files in a window.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After restart, the number of restored windows matches the previous session’s window count in 100% of manual test scenarios (unless constrained by platform limits)
- **SC-002**: After restart, tabs are restored into their original windows with **0 unintended duplicates** in the primary window across the test matrix
- **SC-003**: In a restored multi-window session, the same workspace-discovery error message is not repeated more than once per project per launch
- **SC-004**: Users can complete a restart-and-continue workflow (restart → confirm windows/tabs restored → resume editing) without manual cleanup in 95%+ of test runs
