# Research: Multi-Window Editing (MVP)

## Decisions

### Decision: Share a single `Project` entity across windows in the same project

**Rationale**:

- `project::BufferStore::open_buffer(...)` caches buffers by `ProjectPath`. When two windows share the same `Project` entity, opening the same file returns the same `Buffer` entity, which naturally keeps:
  - text content in sync
  - dirty/unsaved state in sync (tab indicators derive from the shared buffer’s dirty state)

**Alternatives considered**:

- **Separate `Project` per window**: rejected because it would require a bespoke synchronization layer to keep buffers and dirty indicators consistent.

### Decision: Track “active editor window” based on pane focus (not window activation)

**Rationale**:

- `Pane` emits focus events and `Workspace::handle_pane_focused(...)` is invoked when the editor area gains focus.
- Clicking the project tree changes focus to a panel; relying on pane focus prevents project-tree interaction from overwriting the “active editor window” routing target.

**Alternatives considered**:

- **Use OS window activation**: rejected because clicking the project tree in the primary window would make the primary window active even when the user intends opens to go to the secondary editor window.

### Decision: Route project tree opens to the active editor window at the ProjectPanel integration point

**Rationale**:

- `ProjectPanel` already emits `OpenedEntry` events that are handled by the primary window’s `Workspace`, which currently calls `Workspace::open_path_preview(...)` with the primary window context.
- Changing this integration to target a different `WindowHandle<Workspace>` enables routing without rewriting tab/pane logic.

### Decision: Model primary/secondary close semantics as a “project window group”

**Rationale**:

- Secondary close should behave like current `CloseWindow` behavior (remove that window only).
- Primary close must close all secondary windows for that project and avoid losing unsaved work that exists only in secondary windows.
- Coordinating a group close from the primary window provides one place to implement “close secondaries first, then close primary”.

## Constraints / Notes

- MVP avoids introducing new crates; existing collections and `WorkspaceStore` are sufficient to track window-group state.
- Secondary windows should be editor-only in UI chrome. For MVP, the plan focuses on suppressing docks/panels/titlebar content rather than reworking the app menu model (which can be OS-global).

---

## User Story 4: Secondary Window Enhancements (Beyond MVP)

### Decision: Fix Run/Debug actions by ensuring action dispatch context

**Rationale**:

- Run/Debug inline buttons in the editor dispatch actions that require a `Workspace` context.
- In secondary windows, the action dispatch likely fails because either:
  - Actions are not registered in the secondary window's context, OR
  - The action handler looks for UI components (terminal panel, task output) that are suppressed.
- The fix should ensure actions dispatch through the shared `Project` entity, with output appearing in the primary window if needed.

**Research needed**:

- Trace `editor::actions::Run` and `editor::actions::Debug` dispatch path
- Identify if `TaskTerminalPane` or similar is required and how to handle it for secondary windows

### Decision: Minimal status bar shows only essential items

**Rationale**:

- Full status bar includes many items: language selector, diagnostics, git status, copilot, etc.
- For secondary windows, only cursor position (row/column) and Outline Panel toggle are needed.
- Implement as conditional rendering in `StatusBar` based on `WorkspaceWindowRole`.

**Alternatives considered**:

- **Separate `SecondaryStatusBar` component**: rejected as it duplicates logic; better to conditionally render within existing `StatusBar`.

### Decision: Outline Panel uses right dock slot (if enabled) or inline rendering

**Rationale**:

- `OutlinePanel` is currently designed as a dock panel.
- For secondary windows, we can either:
  - Selectively enable the right dock for outline panel only
  - Or render outline panel inline in the workspace layout
- Prefer dock-based approach if feasible to reuse existing panel infrastructure.

**Research needed**:

- Check if `Dock` can be enabled/disabled per-panel rather than all-or-nothing
- Examine `OutlinePanel::new()` and its dependencies

### Decision: Drag-to-monitor uses cursor position at drop time

**Rationale**:

- When a tab is dragged outside the window bounds, the drop position determines target monitor.
- GPUI provides `cx.displays()` to enumerate available displays with bounds.
- Map the drop cursor position to the display containing that point.

**Research needed**:

- Check if GPUI provides drop position coordinates on external window drops
- Verify `Display` API provides monitor bounds in screen coordinates
- Handle edge case: drop between monitors (use closest monitor center)


