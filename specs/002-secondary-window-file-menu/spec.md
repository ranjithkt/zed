# Feature Specification: Secondary Window File Menu Integration

**Feature Branch**: `002-secondary-window-file-menu`  
**Created**: December 26, 2025  
**Status**: Draft  
**Input**: User description: "We just created support to show editor panels in secondary window. It currently only shows title bar without File menu, the tabs which show editors and a status bar at the bottom showing icons for cursor position and toggling Outline Panel. Now I want all the File menu items to work with the secondary window if the focus is in that window. Right now everything in the File menu works on the main window only. You need to figure out where we can just inject the active window so that all other buttons and everything else works on that active window, instead of only the primary window. By design we should aim to use entire existing logic, only difference is they should work on the active window. For example there is a new file created in the secondary window by double clicking on an empty area on the tab panel, then File -> save should work on saving that editor tab in the secondary window when it has focus, rather than working on only the primary window."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Save File in Secondary Window (Priority: P1)

A user creates or edits a file in the secondary window and wants to save it using the File menu or keyboard shortcuts. The save operation should target the active editor in the secondary window, not the main window.

**Why this priority**: This is the most critical operation - users expect save operations to work on the window they're actively working in. Without this, the multi-window feature is severely limited.

**Independent Test**: Can be fully tested by opening a secondary window, creating/editing a file there, and using File > Save or Ctrl+S. Delivers immediate value by enabling basic file operations in secondary windows.

**Acceptance Scenarios**:

1. **Given** a user has an unsaved file open in the secondary window and the secondary window has focus, **When** the user clicks File > Save or presses Ctrl+S, **Then** the active file in the secondary window is saved (not a file in the main window)
2. **Given** a user has multiple unsaved files open across both windows, **When** the user focuses the secondary window and uses File > Save All, **Then** all unsaved files across all project windows are saved (per FR-011)
3. **Given** a user creates a new file in the secondary window by double-clicking the tab panel, **When** the user types content and clicks File > Save, **Then** a save dialog appears for the secondary window's new file

---

### User Story 2 - File Creation and Opening in Secondary Window (Priority: P1)

A user wants to create new files or open existing files in the secondary window using File menu options. These operations should respect the window context where they're initiated.

**Why this priority**: File creation and opening are core workflows. Users need to manage files independently in each window to fully benefit from multi-window support.

**Independent Test**: Can be tested by focusing the secondary window, using File > New or File > Open, and verifying the file appears in the secondary window. Delivers value by enabling independent file management per window.

**Acceptance Scenarios**:

1. **Given** the secondary window has focus, **When** the user clicks File > New, **Then** a new untitled file opens in the secondary window (not the main window)
2. **Given** the secondary window has focus, **When** the user clicks File > Open and selects a file, **Then** the file opens in the secondary window
3. **Given** the user has files open in both windows, **When** the user focuses the secondary window and uses File > Reopen Last Closed, **Then** the last closed file from the secondary window's history is reopened there

---

### User Story 3 - File Closure in Secondary Window (Priority: P2)

A user wants to close individual files or all files in the secondary window using File menu options. Close operations should only affect the window in focus.

**Why this priority**: Essential for file management workflow, but less critical than save operations since users can close tabs directly. Still required for complete File menu functionality.

**Independent Test**: Can be tested by opening multiple files in the secondary window, then using File > Close or File > Close All to verify only secondary window tabs are affected. Delivers value by allowing complete file lifecycle management per window.

**Acceptance Scenarios**:

1. **Given** the secondary window has focus and an active file, **When** the user clicks File > Close or presses Ctrl+W, **Then** only the active file in the secondary window closes
2. **Given** the secondary window has focus with multiple files open, **When** the user clicks File > Close All, **Then** all files in the secondary window close (main window files remain open)
3. **Given** the user closes a file with unsaved changes in the secondary window, **When** prompted, **Then** the save dialog and confirmation are associated with the secondary window

---

### User Story 4 - File Properties and Metadata Operations (Priority: P3)

A user wants to view file properties, copy paths, or perform other metadata operations on files in the secondary window through the File menu.

**Why this priority**: These are less frequently used operations but still necessary for a complete user experience. Users expect all File menu items to respect window context.

**Independent Test**: Can be tested by opening a file in the secondary window and using File menu options like "Copy Path", "Reveal in Explorer", etc. Delivers value by ensuring consistency across all File menu operations.

**Acceptance Scenarios**:

1. **Given** the secondary window has focus with an active file, **When** the user clicks File > Copy Path, **Then** the path of the active file in the secondary window is copied to clipboard
2. **Given** the secondary window has focus with an active file, **When** the user clicks File > Reveal in Explorer, **Then** the file system explorer opens showing the file from the secondary window
3. **Given** the secondary window has focus, **When** the user accesses file properties or metadata operations, **Then** they operate on the secondary window's active file

---

### Edge Cases

The following edge cases are documented for awareness. For MVP, the system uses the `active_editor_window` at action dispatch time; edge cases involving rapid focus transitions follow platform behavior.

- **Focus transition during action**: The system dispatches to whichever window is `active_editor_window` at the moment the action is triggered. Rapid focus changes are handled by the platform's event ordering.
- **Menu opened, focus switched before click**: The action dispatches to the `active_editor_window` at click time, not menu-open time. This matches user expectation ("I'm working in this window now").
- **Focus lost to external app**: When focus returns to Zed, `active_editor_window` is updated on pane focus; File menu actions target the correct window.
- **File open in both windows**: Each window's tab is independent. File > Close closes only the tab in the receiving workspace.
- **Recent files list**: Remains global per project (existing behavior preserved).
- **No files open in focused window**: Actions like Save gracefully handle empty state (no-op or appropriate prompt).
- **Save All progress**: Uses existing save infrastructure; no new progress UI for MVP.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST route all File menu actions to the currently focused window rather than always routing to the main window
- **FR-002**: System MUST apply File > Save operations to the active editor in the focused window
- **FR-003**: System MUST apply File > Save As operations to the active editor in the focused window and show the save dialog modal to that window
- **FR-004**: System MUST open new files in the focused window when File > New is invoked
- **FR-005**: System MUST open selected files in the focused window when File > Open is invoked
- **FR-006**: System MUST close only files in the focused window when File > Close is invoked
- **FR-007**: System MUST close only files in the focused window when File > Close All is invoked
- **FR-008**: System MUST apply File > Reopen Last Closed to the focused window's history
- **FR-009**: System MUST copy the path of the active file in the focused window when File > Copy Path is invoked
- **FR-010**: System MUST reveal the active file from the focused window when File > Reveal in Explorer is invoked
- **FR-011**: System MUST handle File > Save All by saving all unsaved files across all open windows (both main and secondary windows)
- **FR-012**: System MUST maintain proper keyboard shortcut behavior so shortcuts always target the focused window
- **FR-013**: System MUST show file operation dialogs (save, open, close confirmation) as modals attached to the window where the action was invoked (e.g., the primary window if triggered from its File menu), even if the action targets a file in a different window
- **FR-014**: System MUST maintain existing File menu behavior for the main window when it has focus
- **FR-015**: System MUST preserve all existing File menu functionality while adding window-awareness

### Key Entities

- **Active Window**: The window that currently has user focus and should receive File menu actions
- **Editor Tab**: A file editor instance within a window that can be the target of save, close, and other file operations
- **Window Context**: The state and file collection associated with a specific window (main or secondary)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can successfully save files in secondary windows using File menu or keyboard shortcuts, with the operation targeting the correct window 100% of the time
- **SC-002**: Users can create and open files in secondary windows with File menu actions routing to the focused window in 100% of test cases
- **SC-003**: All File menu operations (save, open, close, properties) correctly target the focused window without requiring additional user configuration
- **SC-004**: Users can manage files independently across multiple windows without operations from one window affecting files in another window
- **SC-005**: File operation dialogs (save, open, close confirmations) appear attached to the origin window (where the action was invoked) for 100% of operations, keeping dialogs near the user
- **SC-006**: Zero regression in existing main window File menu behavior when main window has focus
