# Implementation Plan: Multi-Window Editing (MVP)

**Branch**: `[001-multi-window]` | **Date**: 2025-12-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-multi-window/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.cursor/commands/speckit.plan.md` for the execution workflow.

## Summary

Implement editor-only secondary windows for a single project by reusing the existing `Workspace` + `Project` architecture:

- Secondary windows are additional `Workspace` windows that **share the same `Project` entity** as the primary window.
- The project tree remains only in the primary window, but file opens from the project tree are routed to the **active editor window** (tracked by editor/pane focus).
- Because buffers are owned/cached by the shared project (`BufferStore`), opening the same file in multiple windows yields a shared buffer, keeping **content and dirty indicators in sync**.
- Closing behavior is role-aware: closing a secondary window closes only that window; closing the primary window closes the entire project window-group (primary + its secondaries).

## Technical Context

**Language/Version**: Rust 1.92 (`rust-toolchain.toml`)  
**Primary Dependencies**: GPUI (`crates/gpui`), Zed workspace/project abstractions (`crates/workspace`, `crates/project`, `crates/project_panel`)  
**Storage**: Local filesystem + existing workspace persistence (SQLite via `sqlez`) where already used; MVP avoids adding new persistence for secondary windows  
**Testing**: `cargo test` with GPUI tests (`#[gpui::test]`) and existing visual test helpers where applicable  
**Target Platform**: macOS, Windows, Linux (same as existing Zed desktop targets)  
**Project Type**: Rust workspace (multi-crate monorepo)  
**Performance Goals**: Maintain interactive UI responsiveness (target: 60 fps feel) while routing opens across windows  
**Constraints**: No panics in non-test code; propagate errors to UI (per constitution); avoid large refactors  
**Scale/Scope**: MVP scoped to a single project’s primary window + any number of editor-only secondary windows

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Ship small, desired changes**: MVP limited to window roles, open-routing, and shared-buffer window reuse.
- **Rust correctness / no panics**: Plan avoids `unwrap()` in new code paths; error propagation to user prompts.
- **GPUI entity safety**: All cross-window updates use `WindowHandle<T>::update` / `update_in` and avoid re-entrant entity updates.
- **Tests required**: Add targeted tests around shared buffer sync + routing behavior.
- **Repo hygiene**: Prefer extending existing files (`crates/workspace`, `crates/project_panel`) over new modules.

## Project Structure

### Documentation (this feature)

```text
specs/001-multi-window/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── window-group-manager.md
│   ├── project-tree-open-routing.md
│   └── window-close-semantics.md
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
crates/
├── workspace/src/workspace.rs        # Window creation + close handling + focus events
├── project_panel/src/project_panel.rs # Project tree open events
├── project/src/buffer_store.rs       # Shared buffer cache by ProjectPath (enables sync)
├── zed/src/zed.rs                    # Action wiring / window-level behavior
└── zed_actions/src/lib.rs            # Action definitions (if new MVP actions are added)
```

**Structure Decision**: Implement MVP by extending existing Zed crates (`workspace`, `project_panel`, `zed`) and leaning on existing shared-buffer behavior (`project::BufferStore`). Avoid new crates unless a clear gap is discovered.

## Phase 0: Research (completed)

Key findings (see `research.md`):

- `ProjectPanel` opens entries by calling `Workspace::open_path_preview(...)` in the primary window context.
- `Workspace::handle_pane_focused(...)` is a reliable “editor focus” signal (pane focus) that we can use to track the active editor window without being affected by project-tree focus.
- `project::BufferStore::open_buffer(...)` caches by `ProjectPath`; if multiple windows share the same `Project` entity, the same file opens to the same buffer entity → content + dirty sync “for free”.

## Phase 1: Design (MVP)

### Window roles and grouping

- Treat each project’s window set as a **window group** with:
  - exactly one **primary** window (project tree + full UI chrome)
  - zero or more **secondary editor** windows (editor-only chrome)
- Implement grouping in `WorkspaceStore` (already global and already tracks all workspace windows).

### Active editor window routing

- Track per-project **active editor window** via pane focus events:
  - update on `Workspace::handle_pane_focused(...)`
  - do not update on project tree focus or other panel focus
- Route project-tree open events to the active editor window’s workspace:
  - if file already open in that window, activate existing tab
  - else open as new tab and activate
  - do not auto-focus the target window (window focus stays with OS / user)

### Cross-window sync (content + dirty indicators)

- Create secondary windows using the **same `Project` entity** as the primary window.
- Rely on existing `BufferStore` caching to share buffers across windows for the same file.
- Ensure tab “dirty” is derived from the shared buffer so all windows display consistent indicators.

### Primary/secondary close semantics

- Closing a **secondary** window uses the existing per-window close flow (prompts as needed) and removes only that window.
- Closing the **primary** window performs a **group close**:
  - attempt to close secondary windows first (so unsaved work only open in secondaries is not lost)
  - if any close is canceled, abort the group close
  - otherwise close the primary last

## Phase 2: Implementation Plan (MVP)

### 1) Add window role and project grouping

- Add `WorkspaceWindowRole` (Primary / SecondaryEditor) stored on `Workspace`.
- Extend `WorkspaceStore` to track:
  - window groups keyed by project entity id
  - the active editor window id per project

### 2) Create “New Editor Window” (secondary) action

- Add a new action (e.g. `workspace::NewEditorWindow`) wired only in the primary window.
- Implement action to open a new GPUI window whose root view is a `Workspace` created with:
  - the same `Project` entity as the primary
  - role = `SecondaryEditor`
  - UI chrome suppressed (no docks / panels / menus inside window content)

### 3) Render editor-only secondary windows

- Update `Workspace::render(...)` to conditionally render based on role:
  - Primary: existing full workspace UI (titlebar item, docks, status bar, notifications)
  - Secondary: editor-only (center pane group + required overlays for tab usability)

### 4) Track active editor window

- In `Workspace::handle_pane_focused(...)`, update the active editor window for this project in `WorkspaceStore`.
- Initialize the active editor window for a project when the primary workspace is created (or on first pane focus).

### 5) Route project tree opens to the active editor window

- In `ProjectPanel`’s handler for `Event::OpenedEntry`, replace the direct call to `workspace.open_path_preview(...)` with:
  - compute `ProjectPath`
  - look up active editor window for the project
  - issue the open call in that target window’s `Workspace` via `WindowHandle<Workspace>::update(...)`
  - keep error prompts user-visible in the primary window if the open fails

### 6) Close semantics

- Modify `Workspace::close_window(...)`:
  - Secondary: existing behavior (close only this window)
  - Primary: orchestrate group close (secondaries first), then close primary

## Testing Plan (MVP)

- Add a GPUI test that opens the same project in two windows that share a `Project` entity and asserts:
  - opening the same `ProjectPath` yields a shared buffer (via buffer identity or by observing that edits propagate)
  - making an edit in one window updates the other window’s view of the same file and dirty state
- Add a test for routing logic:
  - set active editor window to window B
  - simulate project panel open request from window A
  - assert the item is opened/activated in window B’s workspace

## Crates / Dependencies

- No new crates are expected for MVP. If a small helper is needed for window-group bookkeeping, prefer std collections (already used) and existing patterns in `WorkspaceStore`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
