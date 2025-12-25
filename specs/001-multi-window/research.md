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


