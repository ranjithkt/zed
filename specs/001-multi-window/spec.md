# Feature Specification: Multi-Window Editing

**Feature Branch**: `[001-multi-window]`  
**Created**: 2025-12-24  
**Status**: Draft  
**Input**: User description: "MVP: a single primary window contains the project tree and all app-level UI (menus/panels). Secondary windows are editor-only. Project-tree file selection routes to the active editor window with per-window tab reuse. Closing secondary closes only that window; closing primary closes all windows. If a file is open in multiple windows, content and dirty indicators stay in sync."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create an editor-only window and open files into it from the project tree (Priority: P1)

As a developer, I want to create an editor-only secondary window and open files from the project tree into whichever editor window I’m currently working in so I can use multiple windows side-by-side without repeatedly moving focus or rearranging tabs.

**Why this priority**: This defines the core interaction model that makes multi-window usable day-to-day: a single project tree (in the “project window”) can feed tabs into the currently active editor window, including a secondary editor-only window.

**Independent Test**: Can be fully tested by opening a project, creating a secondary editor-only window, switching editor focus between windows, and verifying project tree file opens route to the correct window and reuse existing tabs when present.

**Acceptance Scenarios**:

1. **Given** a project is open, **When** the user creates a secondary editor window, **Then** a new window opens that can host editor tabs and does not show a project tree.
2. **Given** two windows are open for the same project and the active editor window is the second window, **When** the user clicks a file in the project tree shown in the first window, **Then** the file opens in the second window as the active tab.
3. **Given** two windows are open for the same project and the active editor window is the first window, **When** the user clicks a file in the project tree, **Then** the file opens in the first window as the active tab.
4. **Given** a file is already open as a tab in the active editor window, **When** the user selects that same file in the project tree, **Then** the editor switches to the existing tab rather than opening a duplicate tab in that window.
5. **Given** a file is not open in the active editor window but is open in a different window, **When** the user selects that file in the project tree, **Then** the file opens in the active editor window as a new active tab and remains open in the other window.
6. **Given** a secondary editor window is open, **When** the user looks for the project tree in that secondary window, **Then** they see only editor tabs/editors and do not see a project tree panel in that window.
7. **Given** two windows are open, **When** the user closes one window, **Then** the other window remains open and usable.
8. **Given** a secondary editor window is open, **When** the user closes the primary window, **Then** all windows for that project close.
9. **Given** the same file is open in two windows, **When** the user edits the file in one window, **Then** the other window updates to show the same content without requiring a manual refresh.
10. **Given** the same file is open in two windows, **When** the user modifies the file in one window such that it becomes “unsaved”, **Then** the tab header indicators in both windows reflect the unsaved state consistently.

---

### User Story 2 - Switch between windows efficiently (Priority: P2)

As a developer, I want a fast way to switch between open windows so I can stay productive when working across multiple monitors or tasks.

**Why this priority**: The operating system can switch windows, but a built-in switcher makes multi-window workflows faster and more discoverable.

**Independent Test**: Can be tested by opening multiple windows and using a dedicated window-switching mechanism to bring a selected window to the foreground.

**Acceptance Scenarios**:

1. **Given** multiple windows are open, **When** the user uses the window-switching mechanism, **Then** focus moves to the selected window.

---

### User Story 3 - Expand multi-window behavior beyond MVP (Priority: P3)

As a developer, I want multi-window workflows to feel complete so I can rely on them across sessions and advanced workflows.

**Why this priority**: These are important for parity with mature editors, but they are not required for the MVP behavior defined in P1.

**Independent Test**: Can be tested by moving work between windows, restoring a multi-window session, and confirming common actions behave the same regardless of which window is active.

**Acceptance Scenarios**:

1. **Given** a tab is open in one window, **When** the user moves that tab into a different window (or into a newly created window), **Then** the tab appears in the destination window and is removed from the source window.
2. **Given** the user previously had multiple windows open for a project, **When** they restart the application and reopen the project, **Then** they can restore their prior window layout and open tabs.

---

### Edge Cases

- Selecting a file in the project tree when there is no currently active editor window (for example, before any editor has been focused).
- Selecting a file in the project tree when the last active editor window has been closed.
- Selecting a file in the project tree when the active editor window cannot accept new tabs (for example, it is closing).
- Attempting to open a large number of windows or tabs (ensure the feature continues to behave correctly and remains usable).
- Closing the last window while there are unsaved changes in one or more windows.
- Closing the primary window when there are unsaved changes in secondary windows.
- Closing a secondary window while it has the only visible tab for an unsaved file.
- Using multi-window on systems with multiple monitors / differing DPI scaling (ensure text and layout remain readable).

## Requirements *(mandatory)*

### Functional Requirements

#### MVP: Minimum viable multi-window (editor-only secondary windows + project tree routing)

- **FR-001**: Users MUST be able to create a secondary editor window for the current project.
- **FR-002**: The product MUST have exactly one primary window per project that displays the project tree and app-level UI (for example, menus and non-editor panels).
- **FR-003**: Secondary editor windows MUST display only editor tabs/editors and MUST NOT display the project tree, app-level menus, or non-editor panels.
- **FR-004**: The system MUST track an “active editor window” based on which window most recently had editor focus.
- **FR-005**: Selecting a file in the project tree MUST open that file in the active editor window.
- **FR-006**: If the selected file is already open as a tab in the active editor window, the system MUST activate the existing tab in that window.
- **FR-007**: If the selected file is not open in the active editor window, the system MUST open it as a new tab in that window and make it the active tab.
- **FR-008**: A file MUST be allowed to be open in multiple windows at the same time; opening it in one window MUST NOT close it in another window.
- **FR-009**: When a project-tree file selection opens a file into a different window than the one containing the project tree, the active editor window MUST remain the same before and after the open action.
- **FR-010**: Closing a secondary editor window MUST close only that window and MUST NOT close other windows for the same project.
- **FR-010a**: When the last tab in a secondary editor window is closed, the secondary window MUST close automatically.
- **FR-011**: Closing the primary window MUST close all windows for that project (including all secondary editor windows).
- **FR-012**: When a file is open in multiple windows, edits made in one window MUST be reflected in all other windows that have that file open, keeping the visible content in sync.
- **FR-013**: When a file is open in multiple windows, the “unsaved/dirty” state shown in tab headers MUST be consistent across those windows.

#### Next: Core window management (VS Code-like baseline)

- **FR-014**: Users MUST be able to switch to another open window using a dedicated window-switching mechanism.
- **FR-015**: Users SHOULD be able to move an open tab from one window to another, including moving a tab into a newly created window.

#### Later: Session continuity and scale

- **FR-016**: The application SHOULD support restoring a prior multi-window session for a project, including window count and open tabs, when the user opts in.
- **FR-017**: Common actions (open file, close tab, find, save) MUST behave consistently regardless of which window is active.
- **FR-018**: If saving a file would overwrite newer on-disk content (for example, due to edits from another window or external tool), the user MUST be warned before data loss and MUST be given a clear choice to keep their changes or discard/reload.

### Assumptions

- There is exactly one “project window” that displays the project tree for a project.
- Secondary windows are editor-only by design (editor tabs/editors only).
- Each window has its own set of open tabs and active tab selection.
- “Active editor window” is determined by the most recently focused editor area (not by clicking the project tree).
- Closing the primary window is treated as closing the project and therefore closes all project windows.

### Dependencies

- The application can create and manage more than one top-level window on the user’s operating system.
- The project tree can route a file-open request to a different window than the one hosting the project tree.

### Out of Scope (for the MVP in User Story 1)

- Restoring multiple windows automatically on application restart.
- Moving tabs between windows via drag-and-drop.
- Advanced multi-window layouts (for example, multiple tab groups within a window beyond the existing single tab strip behavior).
- Showing a project tree in secondary editor windows.
- Tab management enhancements in secondary windows beyond basic tab open/close/activate (for example, tab reordering).
- Synchronizing cursor position or selection between windows when the same file is open in multiple windows.

### Key Entities *(include if feature involves data)*

- **Project window**: The primary window that shows the project tree and an editor area.
- **Editor window**: A secondary window that shows only editor tabs/editors.
- **Active editor window**: The window that most recently had editor focus and is the target for project-tree file opens.
- **Project**: The set of folders/files the user is working in (the “workspace” context shared across windows).
- **Tab**: An open item within a window that represents a file or other content.
- **Document**: The editable representation of a file, including whether it has unsaved changes and its last known on-disk version.
- **Session**: A restorable snapshot of open windows and their tabs for a project.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can open a file into a secondary editor window using the project tree and window focus in 2 interactions or fewer (focus the editor window, then select the file).
- **SC-002**: In at least 95% of attempts on typical projects, selecting a file in the project tree results in the correct target window showing the file as the active tab within 2 seconds.
- **SC-003**: In user testing, at least 90% of participants can successfully keep two windows with different tab sets and open files into the intended window without assistance.
- **SC-004**: In all supported environments, secondary windows never show a project tree panel during normal use (verified via automated or manual regression checks).
- **SC-005**: When the same file is open in two windows, text edits in one window appear in the other window within 1 second in at least 95% of attempts, and the unsaved/dirty indicator matches in both windows.
