# Tasks: Multi-Window Session Restore

**Input**: Design documents from `C:\\Repos\\zed\\specs\\001-multi-window-session\\`  
**Prerequisites**: `plan.md` (required), `spec.md` (required), plus `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Tests are REQUIRED for these behavior changes unless clearly infeasible; if a test is infeasible, the task must state why.

**Guiding principle (do not reinvent the wheel)**: Tasks MUST avoid inventing new within-window restore rules. Everything that would happen in single-window restore must remain unchanged; only add what’s needed to persist and restore multiple windows without collapsing them.

**Organization**: Tasks are grouped by user story so each story can be implemented and tested independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Each task includes concrete file paths

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm current behavior and exact touchpoints. No behavior changes yet.

- [ ] T001 Confirm current restore flow collapses windows by using path-based `workspace::open_paths(...)` in `crates/zed/src/main.rs` (capture minimal repro from `specs/001-multi-window-session/quickstart.md`)
- [ ] T002 Locate how single-window restore state is selected for a root set (and where “best match” window selection happens) in `crates/workspace/src/persistence.rs` and `crates/workspace/src/workspace.rs`
- [ ] T003 Confirm secondary editor windows can be created without a persisted workspace id and therefore cannot serialize independently in `crates/workspace/src/workspace.rs`
- [ ] T004 Reproduce the rust-analyzer “Failed to discover workspace” issue and locate the restore/open initialization path differences in `crates/project/src/lsp_store.rs` and `crates/project/src/lsp_store/rust_analyzer_ext.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Minimal persistence and restore primitives needed for multi-window restore, without changing single-window behavior.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [ ] T005 Add/adjust serialized workspace metadata needed to persist multiple window snapshots for the same roots (at minimum: stable `workspace_id` per window and a persisted window role) in `crates/workspace/src/persistence/model.rs`
- [ ] T006 Add DB migration + read/write support for the additional serialized workspace metadata in `crates/workspace/src/persistence.rs`
- [ ] T007 Ensure secondary editor windows receive a persisted workspace id at creation time (so they can serialize independently) in `crates/workspace/src/workspace.rs` (e.g., `Workspace::new_editor_window`, `Workspace::open_in_new_editor_window`)
- [ ] T008 Add persistence API to enumerate last-session workspace snapshots as workspace ids ordered by session window stack in `crates/workspace/src/persistence.rs`
- [ ] T009 Add persistence API to load a `SerializedWorkspace` by workspace id (restore-by-id) in `crates/workspace/src/persistence.rs`
- [ ] T010 Preserve existing single-window behavior for “open project normally” when multiple snapshots exist for the same roots (prefer the primary snapshot; do not change dedupe/missing-file/unsaved-buffer rules) in `crates/workspace/src/persistence.rs`
- [ ] T010 Preserve existing single-window behavior for “open project normally” when multiple snapshots exist for the same roots: only add a deterministic tie-break (prefer the primary snapshot) and do not change any within-window restore rules (dedupe, missing/unavailable file handling, unsaved buffer behavior) in `crates/workspace/src/persistence.rs`
- [ ] T011 [P] Add GPUI tests for the new persistence invariants (multiple workspace snapshots per same roots are distinct and enumerable) in `crates/workspace/src/persistence.rs`

**Checkpoint**: Persistence can represent multiple windows for a session; secondary windows serialize independently; enumeration-by-session returns all workspaces in stack order.

---

## Phase 3: User Story 1 - Restore Tabs into Correct Windows (Priority: P1) 🎯 MVP

**Goal**: Restore multi-window sessions without collapsing/merging windows; within-window restore behavior must remain identical to single-window restore.

**Independent Test**: Quickstart scenarios A/A2/B: after restart, windows/tabs are restored into their original windows (or system window tabs), and no secondary-window tabs are collapsed into the primary window.

### Tests for User Story 1 (required) ⚠️

- [ ] T012 [P] [US1] Add GPUI test that restores two persisted workspace snapshots and asserts they open as two independent windows/tabs (no collapsing) in `crates/workspace/src/workspace.rs` (or `crates/workspace/src/persistence.rs` if restore helper lives there)

### Implementation for User Story 1

- [ ] T013 [US1] Implement restore-by-workspace-id in `crates/zed/src/main.rs`: enumerate last-session workspace ids and restore each snapshot directly (no path-based chooser that can collapse windows)
- [ ] T014 [US1] Implement/extend a helper to “open window from serialized workspace snapshot” while reusing existing per-window `Workspace::load_workspace` logic in `crates/workspace/src/workspace.rs`
- [ ] T015 [US1] Ensure the restore flow honors system window tabs via existing platform/setting behavior (no custom tabbing model) in `crates/zed/src/main.rs`
- [ ] T016 [US1] Validate that within-window restore behavior is unchanged by ensuring restore uses the same code path as single-window restore for loading items/tabs in `crates/workspace/src/workspace.rs`
- [ ] T016 [US1] Validate that within-window restore behavior is unchanged by ensuring restore reuses the existing per-window workspace load path (e.g., `Workspace::load_workspace` / existing deserialization paths) and does not introduce new within-window special-case branches for duplicates/missing files/unsaved buffers in `crates/workspace/src/workspace.rs`

**Checkpoint**: Multi-window restore no longer collapses windows into one; single-window behavior remains unchanged within each window.

---

## Phase 4: User Story 2 - Persist Window/Tab State on Close (Priority: P2)

**Goal**: Closing windows updates persisted session state so the next restart restores the most recent multi-window layout.

**Independent Test**: Quickstart scenario C: close secondary window → restart → the closed window does not reappear.

### Tests for User Story 2 (required) ⚠️

- [ ] T017 [P] [US2] Add GPUI test: close a secondary window, then verify last-session enumeration no longer includes that workspace snapshot in `crates/workspace/src/persistence.rs` and/or `crates/workspace/src/workspace.rs`

### Implementation for User Story 2

- [ ] T018 [US2] Ensure closing a window properly detaches it from the session’s “open windows” set (without impacting other windows) in `crates/workspace/src/workspace.rs`
- [ ] T019 [US2] Ensure close-time serialization persists the latest state for both primary and secondary windows (no stale restore) in `crates/workspace/src/workspace.rs` and `crates/workspace/src/persistence.rs`

**Checkpoint**: Closed windows stay closed after restart; remaining windows restore as last seen.

---

## Phase 5: User Story 3 - Rust Language Features Work After Restore (Priority: P2)

**Goal**: Fix the underlying cause of rust-analyzer discovery failures introduced by open/restore flows, without suppressing status messages.

**Independent Test**: Quickstart scenario D: for a Rust project that works when opened normally, restore does not introduce a persistent “Failed to discover workspace”.

### Tests for User Story 3 (required) ⚠️

- [ ] T020 [P] [US3] Add unit test: restore/open initialization builds the same rust-analyzer initialization context as normal “open project” for the covered scenario in `crates/project/src/lsp_store.rs`

### Implementation for User Story 3

- [ ] T021 [US3] Identify why restore/open differs from normal open for rust-analyzer initialization (e.g., timing of worktree readiness, workspace-folder inputs, or project roots) in `crates/project/src/lsp_store.rs`
- [ ] T022 [US3] Implement the fix so restore/open no longer triggers the discovery failure in projects that work when opened normally (no suppression) in `crates/project/src/lsp_store.rs` (and related plumbing as needed)

**Checkpoint**: Restore does not introduce the rust-analyzer discovery error in the covered scenario(s).

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Validation and repo quality gates.

- [ ] T023 Run `specs/001-multi-window-session/quickstart.md` Scenarios A–F manually and record outcomes
- [ ] T024 [P] Run `cargo test -p workspace -p project -p zed` and fix any failures related to this feature
- [ ] T025 [P] Run `./script/clippy.ps1` and fix any new warnings in modified files
- [ ] T026 Ensure no new `unwrap()` / `expect()` were introduced in non-test code paths touched by this feature

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Phase 2
- **US2 (Phase 4)**: Depends on Phase 2 (can proceed after US1 or in parallel once Phase 2 is complete)
- **US3 (Phase 5)**: Depends on Phase 2 (can proceed after US1 or in parallel once Phase 2 is complete)
- **Polish (Phase 6)**: After desired user stories are complete

### User Story Dependencies

- **US1 (P1)**: Independent once Phase 2 is complete
- **US2 (P2)**: Depends on Phase 2; logically follows US1 for easiest validation
- **US3 (P2)**: Depends on Phase 2; can be pursued in parallel with US2 after US1 is stable

### Parallel Opportunities

- Phase 1 tasks are mostly parallelizable across different files.
- In Phase 2, T005/T006 can be done before T008/T009; tests (T011) can start once schema is settled.
- After Phase 2, US2 and US3 can proceed in parallel.

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1 + Phase 2
2. Complete US1 (Phase 3)
3. Validate quickstart scenarios A/A2/B
4. Then proceed to US2 + US3


