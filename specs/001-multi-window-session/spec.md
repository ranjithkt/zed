# Feature Specification: Multi-Window Session Restore

**Feature Branch**: `001-multi-window-session`  
**Created**: December 26, 2025  
**Status**: Draft  
**Input**: User description: "When I close a multi-window session, reopening Zed should restore the same multi-window layout. Everything else (duplicates, missing files, unsaved buffers, remote reconnect UX) should behave exactly like it already does for single-window restore. Only deviate where multi-window requires it."

**Guiding principle (do not reinvent the wheel)**: This feature MUST NOT introduce any new “single-window concerns” (dedupe rules, missing-file behavior, unsaved buffer behavior, remote reconnect UX). It MUST reuse existing single-window behavior, and only add what is necessary to save/restore multiple windows without collapsing them.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Restore Tabs into Correct Windows (Priority: P1)

A user previously worked with multiple windows (a primary window and one or more secondary editor windows), each with its own set of open editor tabs. After restarting Zed, they expect Zed to reopen the same multi-window layout (or system window tabs, depending on their platform/setting), with each window’s tabs restored into that same window.

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

### User Story 3 - Rust Language Features Work After Restore (Priority: P2)

A user opens or restores a window in a Rust project and expects Rust language intelligence to initialize normally (as it does when opening the project without session restore). They should not see “Failed to discover workspace” for projects where Rust language features normally work.

**Why this priority**: This error blocks Rust language features and creates confusion. The correct outcome is to fix the root cause so the error does not occur for valid Rust projects.

**Independent Test**: Can be tested by restoring a Rust project session and verifying Rust language features initialize successfully (no persistent “Failed to discover workspace” caused by restore for projects that work when opened normally).

**Acceptance Scenarios**:

1. **Given** a Rust project where Rust language features work when opening the project normally, **When** the user opens or restores a window for that project, **Then** rust-analyzer successfully discovers the workspace and does not report “Failed to discover workspace” due to restore/open flow
2. **Given** a multi-window session restore for a Rust project where Rust language features work when opening the project normally, **When** the user restarts Zed, **Then** rust-analyzer workspace discovery succeeds in both the primary and secondary windows (where applicable)

---

### Edge Cases

- **Existing single-window behavior applies per window**: For missing files, unsaved buffers, duplicate handling, and remote reconnect behavior, multi-window restore MUST behave exactly as the current single-window restore behavior, applied independently for each restored window.
- **Many windows**: If a platform-imposed limit prevents restoring all windows/tabs, restore proceeds with as many as the platform allows using Zed’s existing error/reporting behavior (no new UX invented by this feature).
- **Corrupt session state**: If session/window state is unreadable, Zed restores as it already does today (at minimum, a usable primary window).

### Assumptions & Dependencies

- **Assumptions**:
  - Session restore is enabled and the user is reopening the app rather than opening a project “fresh”.
  - The feature applies to multi-window projects for both local filesystem and remote-backed projects (WSL/SSH/remote server), and MUST NOT change any within-window behavior compared to single-window restore.
  - Each window restores within its own project origin (local vs a specific remote connection); windows do not restore a mixed-origin set of tabs.
- **Dependencies**:
  - Multi-window support already exists (primary + secondary editor windows).
  - Session persistence exists and can represent window state over restarts.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST restore the previous session’s set of windows (primary plus any secondary editor windows) when the user restarts the app, honoring the user’s platform/setting for system window tabs
- **FR-002**: System MUST restore each window’s tabs/panes into the same window they were previously associated with (i.e., multi-window restore MUST NOT collapse or merge multiple windows into one)
- **FR-003**: System MUST persist window/tab state such that a restart restores the most recent state at close time
- **FR-004**: For all within-window behaviors (duplicate handling, missing/unavailable files, unsaved buffers, remote reconnect UX), System MUST reuse the existing single-window restore behavior without introducing new special cases for multi-window restore
- **FR-005**: System MUST fix the underlying cause of “Failed to discover workspace” appearing due to open/restore flows in Rust projects where language features normally work, and MUST NOT suppress rust-analyzer status messages as a workaround
- **FR-006**: System MUST restore remote-backed windows (WSL/SSH/remote server) using the same reconnect behavior and user prompts as existing single-window restore, preserving per-window separation by project origin

### Key Entities

- **Session Window State**: The persisted representation of which windows existed (including their ordering) in the prior session.
- **Workspace Snapshot**: A persisted snapshot of one window’s UI/editor state (tabs/panes) that is restored independently.
- **Project Origin**: The connection context for a window (local filesystem vs a specific remote environment); windows are restored within their origin.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After restart, restored windows/tabs match the previous session layout (no unexpected collapse of multiple windows into one) in 100% of manual test scenarios
- **SC-002**: After restart, users do not need to manually clean up window/tab placement caused specifically by multi-window restore in 95%+ of test runs
- **SC-003**: In Rust projects where language features work when opening the project normally, restoring the session does not introduce a persistent “Failed to discover workspace” error in 100% of test runs
- **SC-004**: Users can restart → confirm windows/tabs restored → resume editing without extra steps compared to single-window restore in 95%+ of test runs
