# Contract: Secondary Window Status Bar

This contract defines the status bar behavior for secondary editor windows.

## Definitions

- **Primary window**: The window with full status bar (language, git, diagnostics, copilot, etc.).
- **Secondary window**: Editor-only window with minimal status bar.

## Required Behavior

### Cursor Position Display

- The status bar MUST show the current cursor row and column.
- Format: `Ln {row}, Col {col}` (matching primary window format).
- Updates MUST occur within 100ms of cursor movement.
- If no editor is active, display should be blank or show a placeholder.

### Outline Panel Toggle

- The status bar MUST include a toggle button for the Outline Panel.
- Icon: Use existing outline/document-structure icon.
- Click behavior: Toggle visibility of the Outline Panel in the secondary window.
- Visual state: Button should indicate whether panel is currently visible.

### Excluded Items

The secondary window status bar MUST NOT show:

- Language selector
- Git branch/status
- Diagnostics count
- Copilot status
- Extension status items
- Breadcrumbs
- Any other full status bar items

## Implementation Notes

- Implement as conditional rendering within existing `StatusBar` component.
- Check `workspace.role == WorkspaceWindowRole::SecondaryEditor` to enable minimal mode.
- Subscribe to active editor's cursor position updates.

