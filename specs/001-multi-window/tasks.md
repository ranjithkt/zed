# Tasks: Multi-Window Editing (MVP)

**Input**: Design documents from `specs/001-multi-window/`  
**Prerequisites**: `specs/001-multi-window/plan.md`, `specs/001-multi-window/spec.md`, `specs/001-multi-window/research.md`, `specs/001-multi-window/data-model.md`, `specs/001-multi-window/contracts/*`

**Tests**: Tests are REQUIRED for behavior changes. Where full end-to-end coverage is hard, add focused GPUI tests around the new routing/grouping logic.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish a clean baseline and ensure the existing test harness is working before introducing multi-window behavior.

- [x] T001 Run baseline tests for touched crates: `cargo test -p workspace -p project_panel` (repo root: `Cargo.toml`)
- [x] T002 Run baseline linting for touched code: `./script/clippy` (repo root: `script/clippy`)
- [x] T003 [P] Identify current call sites for window creation + project tree open (targets: `crates/workspace/src/workspace.rs`, `crates/project_panel/src/project_panel.rs`, `crates/zed/src/zed.rs`)
 

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add the minimal internal primitives that all MVP work depends on: window roles, window grouping per project, and active-editor-window tracking.

**⚠️ CRITICAL**: No user story implementation should proceed until this phase is complete.

- [x] T004 Add `WorkspaceWindowRole` + `role` field on `Workspace` with default `Primary` in `crates/workspace/src/workspace.rs`
- [x] T005 [P] Add `ProjectKey` and `WorkspaceWindowGroup` types in `crates/workspace/src/workspace.rs`
- [x] T006 Extend `WorkspaceStore` to track window groups + active editor window per `ProjectKey` in `crates/workspace/src/workspace.rs`
- [x] T007 Wire `Workspace` creation/destruction to register/unregister its `WindowId` in `WorkspaceStore` in `crates/workspace/src/workspace.rs`
- [x] T008 Update `Workspace::handle_pane_focused(...)` to record the active editor window for the project in `WorkspaceStore` in `crates/workspace/src/workspace.rs`
- [x] T009 Add `WorkspaceStore` query helpers (`active_editor_window_for_project`, `primary_window_for_project`, `secondary_windows_for_project`) in `crates/workspace/src/workspace.rs`
- [x] T010 [P] Add test-only helpers to create two `Workspace` windows sharing the same `Project` and `AppState` in `crates/workspace/src/workspace.rs`

**Checkpoint**: Foundation ready — multi-window behavior can now be implemented and tested.

---

## Phase 3: User Story 1 - Editor-only secondary windows + project tree routing (Priority: P1) 🎯 MVP

**Goal**: Users can create editor-only secondary windows for a project; project tree opens route to the active editor window; the same file can be open in multiple windows with content + dirty indicators staying in sync; primary-close closes the entire window group.

**Independent Test**: Follow `specs/001-multi-window/quickstart.md` end-to-end and confirm all acceptance scenarios under User Story 1.

### Tests for User Story 1 (required for behavior changes) ⚠️

> Write these tests early so they fail (or demonstrate missing behavior) before the full implementation lands.

- [x] T011 [P] [US1] Add GPUI test for cross-window sync (shared buffer + dirty state) in `crates/workspace/src/workspace.rs`
- [x] T012 [P] [US1] Add GPUI test for active-editor-window routing from project tree open event in `crates/project_panel/src/project_panel_tests.rs`
- [x] T013 [P] [US1] Add GPUI test for close semantics: secondary closes self; primary closes all windows in group in `crates/workspace/src/workspace.rs`

### Implementation for User Story 1

- [x] T014 [US1] Add `workspace::NewEditorWindow` action in `crates/workspace/src/workspace.rs`
- [x] T015 [US1] Register/handle `NewEditorWindow` in `crates/zed/src/zed.rs` to open a new GPUI window whose `Workspace` shares the same `Project` as the primary window
- [x] T016 [US1] Mark the newly created window's workspace role as `SecondaryEditor` and ensure the primary window remains `Primary` in `crates/workspace/src/workspace.rs`
- [x] T017 [US1] Render secondary windows as editor-only (no project tree, no non-editor panels, no app-level chrome inside window content) in `crates/workspace/src/workspace.rs`
- [x] T017a [US1] Add "Open in New Editor Window" to project panel right-click context menu in `crates/project_panel/src/project_panel.rs`
- [x] T017b [US1] Fix bug in `new_editor_window` async closure - correct error handling pattern for `cx.open_window` in `crates/workspace/src/workspace.rs`
- [x] T018 [US1] Update `ProjectPanel` open handling to route to the active editor window (lookup via `WorkspaceStore`) in `crates/project_panel/src/project_panel.rs`
- [x] T019 [US1] Ensure per-window tab reuse: opening a file already open in the target window activates the existing tab (verify/adjust via `Pane::open_item` usage) in `crates/workspace/src/workspace.rs`
- [x] T020 [US1] Ensure project-tree interaction does not overwrite active editor window selection (active editor window updates only on pane focus) in `crates/workspace/src/workspace.rs`
- [x] T021 [US1] Implement primary/secondary close semantics: primary triggers group-close (secondaries first, abort if canceled), secondary closes only itself in `crates/workspace/src/workspace.rs`
- [x] T021a [US1] Implement auto-close for secondary windows when their last tab is closed in `crates/workspace/src/workspace.rs`
- [x] T022 [US1] Ensure open errors surfaced as prompts in the primary window even when routing opens to another window in `crates/project_panel/src/project_panel.rs`
- [x] T023 [US1] Update quickstart command names/steps if they differ from the final action wiring in `specs/001-multi-window/quickstart.md`

### Validation

- [x] T024 [US1] Run focused tests after implementation: `cargo test -p workspace -p project_panel` (repo root: `Cargo.toml`)
- [x] T025 [US1] Run quickstart manual validation checklist in `specs/001-multi-window/quickstart.md`

**Checkpoint**: MVP complete — multi-window behavior is functional and testable independently.

**Known Bugs** (to address post-MVP):
- ⚠️ BUG-001: "window not found" errors appear in terminal when secondary window closes (race condition with Windows platform activation events). Non-blocking but should be investigated.

---

## Phase 4: User Story 2 - Switch between windows efficiently (Priority: P2)

**Goal**: Provide a dedicated window-switching mechanism beyond OS switching.

**Independent Test**: Open multiple windows and switch focus via the dedicated mechanism.

> Note: The current implementation plan is MVP-only. These tasks capture the next-step work needed to plan and implement US2 without expanding the MVP scope.

- [ ] T026 [US2] Audit existing window switch actions (`ActivateNextWindow` / `ActivatePreviousWindow`) and their handlers in `crates/workspace/src/workspace.rs`
- [ ] T027 [US2] Extend `specs/001-multi-window/plan.md` with a concrete US2 design (UI entry point + keybinding expectations) in `specs/001-multi-window/plan.md`

---

## Phase 5: User Story 3 - Expand multi-window beyond MVP (Priority: P3)

**Goal**: Add post-MVP capabilities (tab moving, session restore, etc.).

**Independent Test**: Validate each added capability independently (tab move, restore, etc.).

> Note: The current implementation plan is MVP-only. These tasks capture planning work for later phases.

- [ ] T028 [US3] Add a follow-up design section for tab moving + session restore in `specs/001-multi-window/plan.md`

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finish quality gates and reduce regression risk.

- [ ] T029 [P] Re-run `./script/clippy` after implementation changes (repo root: `script/clippy`)
- [ ] T030 Ensure the spec checklist remains accurate after implementation decisions in `specs/001-multi-window/checklists/requirements.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)** → **Phase 2 (Foundational)** → **Phase 3 (US1 / MVP)**
- **US2/US3 phases** are optional follow-ups and should not block MVP delivery.

### User Story Dependencies

- **US1** depends on Phase 2 only.
- **US2** and **US3** should build on the MVP but can be planned/implemented later.

### Parallel Opportunities

- Tasks marked **[P]** can be worked in parallel (different files / low coupling).
- Within US1, test writing tasks (T011–T013) can proceed in parallel with foundational scaffolding (T004–T010) once naming/entry points are agreed.

---

## Parallel Example: User Story 1

```bash
# Parallelizable (different files):
# - Write sync test in workspace
# - Write routing test in project_panel
# - Implement WorkspaceStore grouping primitives
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phases 1–2 (baseline + foundational primitives)
2. Complete Phase 3 (US1) and verify with:
   - `cargo test -p workspace -p project_panel`
   - `specs/001-multi-window/quickstart.md`
3. Stop and evaluate before starting US2/US3 planning/implementation.


