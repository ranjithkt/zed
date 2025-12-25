# Contract: Drag Tab to Monitor

This contract defines the behavior when a user drags a tab from the primary window to a different monitor.

## Definitions

- **Source window**: The window from which the tab is being dragged.
- **Target monitor**: The display/monitor where the tab is dropped.
- **Drop position**: The cursor position in screen coordinates when the drag ends.

## Required Behavior

### Drag Detection

- When a tab drag ends outside any existing window bounds, the system MUST detect this as an external drop.
- The drop position MUST be captured in screen coordinates.

### Monitor Detection

- The system MUST query available displays using GPUI's display enumeration.
- The system MUST find the display containing the drop position.
- If the drop position is between monitors (in a gap), the system SHOULD use the nearest monitor.

### Window Creation

- A new secondary editor window MUST be created on the target monitor.
- The window MUST be positioned near the drop position (centered or slightly offset).
- The window MUST be sized appropriately for the monitor (respect DPI scaling).
- The dragged tab MUST be opened in the new window.

### Tab Handling

- The tab SHOULD be removed from the source window after successful creation.
- If window creation fails, the tab MUST remain in the source window.
- If the dragged item was the only tab in a secondary window, that window SHOULD close (per FR-010a).

## Edge Cases

### Drop on Same Monitor

- If the drop is on the same monitor as the source window, create a new window on that monitor.
- Position the new window offset from the source to avoid complete overlap.

### Drop Between Monitors

- Use the monitor whose center is closest to the drop position.
- Or use the monitor that contains the majority of the drop area.

### Different DPI Scaling

- The new window MUST respect the target monitor's DPI scaling.
- Window size should be scaled appropriately for the target monitor.

## Implementation Notes

- Extend `Pane` drag-drop handling to detect external drops.
- Use `cx.displays()` to enumerate monitors.
- Use `Display::bounds()` to determine monitor boundaries.
- Reuse `new_editor_window` action logic for window creation.

