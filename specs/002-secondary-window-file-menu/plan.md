# Implementation Plan: Secondary Window File Menu Integration

**Branch**: `002-secondary-window-file-menu` | **Date**: 2025-12-26 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/002-secondary-window-file-menu/spec.md`

## Summary

Make **File menu actions** (Save, Save As, Save All, New, Open, Close Editor, Close Window, etc.) operate on the **editor window that actually has focus** (including secondary editor windows), even though secondary windows intentionally do not render an application menu. Reuse the existing multi-window grouping logic by routing File menu action dispatch to the project group’s `active_editor_window`.

## Technical Context

**Language/Version**: Rust 1.92 (see `rust-toolchain.toml`)  
**Primary Dependencies**: `gpui`, `workspace`, `title_bar`, `ui`  
**Storage**: N/A (no new persistence)  
**Testing**: `cargo test` (GPUI tests via `#[gpui::test]`)  
**Target Platform**: Desktop (Windows/macOS/Linux)  
**Project Type**: Rust workspace (multi-crate desktop app)  
**Performance Goals**: Menu action routing should add no perceptible latency to user actions  
**Constraints**: Avoid panics, propagate errors to user-visible prompts, and keep changes narrowly scoped  
**Scale/Scope**: Focused changes to client-side application menu dispatch + any per-action fixes needed to meet spec (File menu behavior)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Scope discipline**: Limit changes to File menu action routing and directly related action handlers; avoid unrelated refactors.
- **Panic avoidance**: No `unwrap` / `expect` in non-test code. Use `?` or explicit handling; log intentional ignores.
- **GPUI correctness**: Use current `Entity<T>`, `App`, `Context<T>` APIs; avoid re-entrant entity updates.
- **Async/task hygiene**: Don’t drop tasks that must complete; use `detach_and_log_err` / `detach_and_prompt_err` as appropriate.
- **Tests**: Behavior changes should be covered with deterministic tests (use GPUI executor timers if needed).

✅ No planned violations.

## Project Structure

### Documentation (this feature)

```text
specs/002-secondary-window-file-menu/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── action-routing-window-context.md
└── tasks.md             # Created by /speckit.tasks (next step)
```

### Source Code (repository root)

```text
crates/title_bar/src/
├── title_bar.rs               # Secondary windows skip app menu; primary hosts menu
└── application_menu.rs        # Builds menu UI and dispatches actions (routing hook point)

crates/zed/src/
└── zed/app_menus.rs           # Defines File menu items -> action types

crates/workspace/src/
└── workspace.rs               # Tracks per-project active editor window; implements most File actions
```

**Structure Decision**: This is a Rust multi-crate desktop application. The feature is implemented by adjusting **client-side menu action dispatch** (in `title_bar`) to target the correct workspace window using existing `WorkspaceStore` window-group state, plus fixing any File actions that still incorrectly assume primary.

## Complexity Tracking

N/A (no constitution violations)

## Phase 0: Research (completed)

See [research.md](research.md).

## Phase 1: Design & Contracts (produced by this plan)

- **Data model**: [data-model.md](data-model.md)
- **Action routing contract**: [contracts/action-routing-window-context.md](contracts/action-routing-window-context.md)
- **Manual verification**: [quickstart.md](quickstart.md)

## Phase 2: Task Breakdown (next)

Run `/speckit.tasks` to break the contract into implementation tasks in `tasks.md`.
