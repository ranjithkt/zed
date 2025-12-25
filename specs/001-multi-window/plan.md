# Implementation Plan: Multi-Window Editing (MVP + Enhancements)

**Branch**: `[001-multi-window]` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-multi-window/spec.md`

**Note**: This plan extends the completed MVP with User Story 4 (secondary window enhancements).

## Summary

### MVP (Complete ✅)

Implement editor-only secondary windows for a single project by reusing the existing `Workspace` + `Project` architecture:

- Secondary windows are additional `Workspace` windows that **share the same `Project` entity** as the primary window.
- The project tree remains only in the primary window, but file opens from the project tree are routed to the **active editor window** (tracked by editor/pane focus).
- Because buffers are owned/cached by the shared project (`BufferStore`), opening the same file in multiple windows yields a shared buffer, keeping **content and dirty indicators in sync**.
- Closing behavior is role-aware: closing a secondary window closes only that window; closing the primary window closes the entire project window-group (primary + its secondaries).

### Beyond MVP: User Story 4 (Secondary Window Enhancements)

Extend secondary windows with essential productivity features:

1. **Run/Debug actions** (FR-019): Fix bug where Run/Debug buttons beside `main()` functions don't execute from secondary windows.
2. **Minimal status bar** (FR-020): Show cursor row/column position at the bottom of secondary windows.
3. **Outline Panel toggle** (FR-021): Add status bar button to show/hide Outline Panel for document structure navigation.
4. **Drag tab to monitor** (FR-022): When dragging a tab to a different monitor, open a new secondary window on that monitor.

## Technical Context

**Language/Version**: Rust 1.92 (`rust-toolchain.toml`)  
**Primary Dependencies**: GPUI (`crates/gpui`), Zed workspace/project abstractions (`crates/workspace`, `crates/project`, `crates/project_panel`, `crates/outline_panel`, `crates/editor`)  
**Storage**: Local filesystem + existing workspace persistence (SQLite via `sqlez`) where already used  
**Testing**: `cargo test` with GPUI tests (`#[gpui::test]`) and existing visual test helpers where applicable  
**Target Platform**: macOS, Windows, Linux (same as existing Zed desktop targets)  
**Project Type**: Rust workspace (multi-crate monorepo)  
**Performance Goals**: Maintain interactive UI responsiveness (target: 60 fps feel); status bar updates within 100ms; drag-drop window creation within 500ms  
**Constraints**: No panics in non-test code; propagate errors to UI (per constitution); avoid large refactors  
**Scale/Scope**: Single project's primary window + any number of secondary editor windows with enhanced features

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Ship small, desired changes**: Each feature (Run/Debug fix, status bar, outline panel, drag-to-monitor) is narrowly scoped.
- **Rust correctness / no panics**: Plan avoids `unwrap()` in new code paths; error propagation to user prompts.
- **GPUI entity safety**: All cross-window updates use `WindowHandle<T>::update` / `update_in` and avoid re-entrant entity updates.
- **Tests required**: Add targeted tests around new behaviors.
- **Repo hygiene**: Prefer extending existing files (`crates/workspace`, `crates/outline_panel`) over new modules.

## Project Structure

### Documentation (this feature)

```text
specs/001-multi-window/
├── spec.md
├── plan.md
├── research.md          # Updated with US4 research
├── data-model.md        # Updated with US4 entities
├── quickstart.md
├── contracts/
│   ├── window-group-manager.md
│   ├── project-tree-open-routing.md
│   ├── window-close-semantics.md
│   ├── secondary-status-bar.md      # NEW
│   └── drag-tab-to-monitor.md       # NEW
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
crates/
├── workspace/src/workspace.rs        # Window creation, close handling, status bar rendering
├── project_panel/src/project_panel.rs # Project tree open events
├── project/src/buffer_store.rs       # Shared buffer cache (existing)
├── outline_panel/src/outline_panel.rs # Outline panel (adapt for secondary windows)
├── editor/src/editor.rs              # Cursor position events for status bar
├── zed/src/zed.rs                    # Action wiring / window-level behavior
└── zed_actions/src/lib.rs            # Action definitions
```

**Structure Decision**: Implement by extending existing Zed crates. No new crates needed.

## Phase 0: Research (US4 Additions)

### Research: Run/Debug actions in secondary windows

**Question**: Why don't Run/Debug inline buttons work in secondary windows?

**Investigation approach**:
- Trace the action dispatch path for `Run` and `Debug` actions from editor inline buttons
- Check if actions require primary window context or specific workspace state
- Identify where the action routing breaks for secondary windows

**Expected findings**:
- Actions may be registered only in primary window's action context
- Or actions may require panels/UI that are suppressed in secondary windows

### Research: Status bar architecture

**Question**: How does the existing status bar work, and how to render a minimal version in secondary windows?

**Investigation approach**:
- Examine `crates/workspace/src/status_bar.rs` and `StatusBar` component
- Identify how cursor position is communicated from editor to status bar
- Determine minimal status bar elements needed (row/column + outline toggle)

### Research: Outline Panel in secondary windows

**Question**: Can the existing Outline Panel work in secondary windows?

**Investigation approach**:
- Examine `crates/outline_panel` and how it integrates with `Workspace`
- Check if outline panel requires docks (which are suppressed in secondary windows)
- Identify alternative mounting approach for secondary windows

### Research: Drag tab to monitor detection

**Question**: How to detect which monitor a tab is dragged to?

**Investigation approach**:
- Examine existing tab drag-drop implementation in `crates/workspace/src/pane.rs`
- Check GPUI's window positioning and monitor detection APIs
- Identify platform-specific considerations (Windows, macOS, Linux)

**Output**: research.md updated with US4 decisions

## Phase 1: Design (US4)

### Run/Debug actions fix

- Ensure Run/Debug actions are registered in secondary window's action context
- Actions should dispatch to the shared `Project` entity's task runner
- No UI changes needed; just fix action routing

### Secondary window status bar

- Add a minimal `StatusBar` variant for secondary windows:
  - Show only: cursor row/column (from active editor)
  - Show only: Outline Panel toggle button
  - Suppress: all other status bar items (language, git, diagnostics, etc.)
- Update `Workspace::render(...)` to conditionally render minimal status bar for secondary windows

### Outline Panel integration

- For secondary windows, mount Outline Panel differently:
  - Option A: Use a right-side dock (if docks can be selectively enabled)
  - Option B: Render inline as part of workspace content (beside editor)
- Outline Panel subscribes to active editor changes in the workspace
- Panel content updates when active tab changes

### Drag tab to monitor

- Extend tab drag-drop handling in `Pane`:
  - On drop outside current window bounds, detect target monitor
  - Create new secondary window positioned on that monitor
  - Transfer the dragged item to the new window
- Use GPUI's `cx.displays()` to enumerate monitors and determine drop target

## Phase 2: Implementation Plan (US4)

### 1) Fix Run/Debug actions in secondary windows

- Investigate action dispatch in `crates/editor/src/editor.rs` for inline run buttons
- Ensure actions are wired in secondary window's `Workspace`
- Test: Run/Debug from secondary window triggers task execution

### 2) Implement minimal status bar for secondary windows

- Create `SecondaryStatusBar` component or conditional rendering in `StatusBar`
- Subscribe to active editor's cursor position via `Editor::cursor_position()`
- Add Outline Panel toggle button with icon
- Wire toggle to show/hide Outline Panel in secondary window

### 3) Implement Outline Panel for secondary windows

- Modify `Workspace::render()` to optionally show Outline Panel for secondary windows
- Reuse existing `OutlinePanel` component
- Subscribe to workspace's active item changes
- Test: Outline Panel shows correct structure; updates on tab switch

### 4) Implement drag tab to monitor

- Extend `Pane` drag-drop to detect external drops
- On external drop, get cursor position and map to monitor
- Create secondary window on target monitor with dragged item
- Test: Drag tab to second monitor creates window there

## Testing Plan (US4)

- Add GPUI test for Run/Debug action dispatch from secondary window
- Add GPUI test for status bar cursor position updates in secondary window
- Add GPUI test for Outline Panel toggle and content updates
- Add GPUI test for drag-to-monitor window creation (may require mock display setup)

## Crates / Dependencies

- No new crates needed
- May need to expose additional GPUI APIs for monitor detection if not already available

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
