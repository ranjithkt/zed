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

## Phase 4: User Story 4 - Secondary Window Enhancements (Priority: P2) 🎯

**Goal**: Extend secondary windows with essential productivity features: Run/Debug actions, minimal status bar with cursor position, Outline Panel toggle, and drag-tab-to-monitor window creation.

**Independent Test**: Follow `specs/001-multi-window/quickstart.md` User Story 4 section to verify all acceptance scenarios.

### Research for User Story 4

- [x] T031 [P] [US4] Investigate Run/Debug action dispatch path from editor inline buttons in `crates/editor/src/editor.rs`
- [x] T032 [P] [US4] Examine existing status bar architecture in `crates/workspace/src/status_bar.rs`
- [x] T033 [P] [US4] Examine OutlinePanel integration with Workspace in `crates/outline_panel/src/outline_panel.rs`
- [x] T034 [P] [US4] Investigate tab drag-drop and GPUI display APIs in `crates/workspace/src/pane.rs` and `crates/gpui/src`

### Tests for User Story 4 (required for behavior changes) ⚠️

- [ ] T035 [P] [US4] Add GPUI test for Run/Debug action dispatch from secondary window in `crates/workspace/src/workspace.rs`
- [ ] T036 [P] [US4] Add GPUI test for status bar cursor position updates in secondary window in `crates/workspace/src/workspace.rs`
- [ ] T037 [P] [US4] Add GPUI test for Outline Panel toggle and content updates in secondary window in `crates/workspace/src/workspace.rs`
- [ ] T038 [P] [US4] Add GPUI test for drag-to-monitor window creation in `crates/workspace/src/pane.rs`

### Implementation: FR-019 - Run/Debug actions in secondary windows

- [x] T039 [US4] Identify why Run/Debug actions fail in secondary windows (trace action dispatch) in `crates/editor/src/editor.rs`
  - Root cause: `terminal_provider` is None in secondary windows (no TerminalPanel)
  - Solution: Route task/debug spawning to primary window via `WorkspaceStore`
- [x] T040 [US4] Ensure Run/Debug actions are registered in secondary window's action context in `crates/workspace/src/tasks.rs`
  - Added `spawn_task_via_primary_window()` for task routing
  - Added `start_debug_via_primary_window()` for debug session routing
- [ ] T041 [US4] Verify task output appears correctly (in primary window terminal if needed) - manual testing required

### Implementation: FR-020 - Minimal status bar

- [x] T042 [US4] Add conditional rendering for secondary window status bar in `crates/zed/src/zed.rs`
  - Modified `initialize_workspace` to check `workspace.role()` and add only cursor position for secondary windows
  - Fixed dock buttons in `workspace.rs` to only be added for primary windows (was causing non-functional icons)
- [x] T043 [US4] Implement cursor row/column display component for secondary status bar
  - Reused existing `CursorPosition` component from `go_to_line` crate
- [x] T044 [US4] Subscribe to active editor cursor position changes
  - Already handled by existing `CursorPosition` component's `StatusItemView` impl
- [x] T045 [US4] Update `Workspace::render()` to show minimal status bar for `SecondaryEditor` role in `crates/workspace/src/workspace.rs`
  - Changed `status_bar_visible()` to show status bar for all windows (was primary-only)
  - Added `TitleBar::new_minimal()` for secondary windows - shows window controls but no File menu

### Implementation: FR-021 - Outline Panel toggle

- [ ] T046 [US4] Add Outline Panel toggle button to secondary status bar in `crates/workspace/src/status_bar.rs`
- [ ] T047 [US4] Add `outline_panel_visible` state to secondary workspace in `crates/workspace/src/workspace.rs`
- [ ] T048 [US4] Implement Outline Panel mounting for secondary windows (dock or inline) in `crates/workspace/src/workspace.rs`
- [ ] T049 [US4] Wire OutlinePanel to secondary window's active item changes in `crates/outline_panel/src/outline_panel.rs`
- [ ] T050 [US4] Ensure Outline Panel updates when switching tabs in secondary window in `crates/outline_panel/src/outline_panel.rs`

### Implementation: FR-022 - Drag tab to monitor

- [ ] T051 [US4] Extend Pane drag-drop to detect drops outside window bounds in `crates/workspace/src/pane.rs`
- [ ] T052 [US4] Implement monitor detection using `cx.displays()` API in `crates/workspace/src/pane.rs`
- [ ] T053 [US4] Create secondary window on target monitor with dragged item in `crates/workspace/src/workspace.rs`
- [ ] T054 [US4] Handle edge case: drop between monitors (use nearest monitor) in `crates/workspace/src/pane.rs`
- [ ] T055 [US4] Handle DPI scaling differences between monitors in `crates/workspace/src/workspace.rs`

### Validation for User Story 4

- [ ] T056 [US4] Run focused tests: `cargo test -p workspace -p editor -p outline_panel` (repo root)
- [ ] T057 [US4] Run quickstart manual validation (US4 section) in `specs/001-multi-window/quickstart.md`

**Checkpoint**: User Story 4 complete — secondary windows have Run/Debug, status bar, outline panel, and drag-to-monitor features.

---

## Phase 5: User Story 2 - Switch between windows efficiently (Priority: P2)

**Goal**: Provide a dedicated window-switching mechanism beyond OS switching.

**Independent Test**: Open multiple windows and switch focus via the dedicated mechanism.

> Note: Tasks capture next-step work to plan and implement US2.

- [ ] T058 [US2] Audit existing window switch actions (`ActivateNextWindow` / `ActivatePreviousWindow`) and their handlers in `crates/workspace/src/workspace.rs`
- [ ] T059 [US2] Extend `specs/001-multi-window/plan.md` with a concrete US2 design (UI entry point + keybinding expectations) in `specs/001-multi-window/plan.md`

---

## Phase 6: User Story 3 - Expand multi-window beyond MVP (Priority: P3)

**Goal**: Add post-MVP capabilities (tab moving, session restore, etc.).

**Independent Test**: Validate each added capability independently (tab move, restore, etc.).

> Note: Tasks capture planning work for later phases.

- [ ] T060 [US3] Add a follow-up design section for tab moving + session restore in `specs/001-multi-window/plan.md`

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Finish quality gates and reduce regression risk.

- [ ] T061 [P] Re-run `./script/clippy` after implementation changes (repo root: `script/clippy`)
- [ ] T062 Ensure the spec checklist remains accurate after implementation decisions in `specs/001-multi-window/checklists/requirements.md`
- [ ] T063 [P] Investigate and fix BUG-001 (window not found errors on secondary close) in `crates/workspace/src/workspace.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)** → **Phase 2 (Foundational)** → **Phase 3 (US1 / MVP)** ✅ Complete
- **Phase 4 (US4)**: Can start immediately (builds on MVP foundation)
- **Phase 5 (US2)** and **Phase 6 (US3)**: Optional follow-ups

### User Story Dependencies

- **US1** ✅ Complete
- **US4** depends on US1 completion (uses secondary window infrastructure)
- **US2** and **US3** can be planned/implemented after US4

### Parallel Opportunities within US4

- Research tasks (T031–T034) can run in parallel
- Test tasks (T035–T038) can run in parallel after research
- FR-019 (Run/Debug), FR-020 (Status Bar), FR-021 (Outline Panel), FR-022 (Drag-to-monitor) can be worked in parallel

---

## Parallel Example: User Story 4

```bash
# Research phase (parallelizable - different files):
# - T031: Investigate Run/Debug in editor
# - T032: Examine status bar architecture
# - T033: Examine outline panel integration
# - T034: Investigate drag-drop APIs

# Implementation (parallelizable - different features):
# - FR-019: Run/Debug fix (T039-T041)
# - FR-020: Status bar (T042-T045)
# - FR-021: Outline Panel (T046-T050)
# - FR-022: Drag-to-monitor (T051-T055)
```

---

## Implementation Strategy

### Current Focus: User Story 4

1. ✅ MVP Complete (Phases 1–3)
2. Complete Phase 4 (US4) and verify with:
   - `cargo test -p workspace -p editor -p outline_panel`
   - `specs/001-multi-window/quickstart.md` (US4 section)
3. Evaluate before starting US2/US3.


