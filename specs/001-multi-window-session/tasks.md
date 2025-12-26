# Tasks: Multi-Window Session Restore

**Input**: Design documents from `/specs/001-multi-window-session/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/session-restore.md`, `quickstart.md`

**Tests**: Tests are REQUIRED for these behavior changes unless a task explicitly notes why a deterministic automated test is not feasible.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm current behavior and identify exact code touchpoints; no new project structure.

- [ ] T001 Confirm current restore flow uses `workspace::open_paths(...)` in `crates/zed/src/main.rs` and document a minimal reproduction from `specs/001-multi-window-session/quickstart.md`
- [ ] T002 Locate how secondary editor windows are created (`Workspace::new_with_role(None, ..., SecondaryEditor, ...)`) and confirm they currently lack `database_id` in `crates/workspace/src/workspace.rs`
- [ ] T003 Locate rust-analyzer status notification logging in `crates/project/src/lsp_store/rust_analyzer_ext.rs` and capture the conditions under which it reports “Failed to discover workspace” for a valid Rust workspace

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Persistence + restore primitives that MUST exist before any story can be verified.

- [ ] T004 Add persisted `window_role` to serialized workspace model in `crates/workspace/src/persistence/model.rs` (Primary vs SecondaryEditor)
- [ ] T005 Add DB migration + read/write support for persisted `window_role` in `crates/workspace/src/persistence.rs` (schema + serialization/deserialization)
- [ ] T006 Add persistence API to list last-session workspaces as `(workspace_id, location, paths, window_id, window_role)` ordered by the saved window stack in `crates/workspace/src/persistence.rs` (must include local + remote-backed workspaces)
- [ ] T007 Add persistence API to load a `SerializedWorkspace` by `workspace_id` in `crates/workspace/src/persistence.rs`
- [ ] T008 Ensure secondary editor windows receive a persisted `WorkspaceId` (database id) at creation time in `crates/workspace/src/workspace.rs` (`Workspace::new_editor_window`, `Workspace::open_in_new_editor_window`)
- [ ] T009 Ensure `Workspace::serialize_workspace_internal` persists `session_id`, `window_id`, and `window_role` consistently in `crates/workspace/src/workspace.rs` for both local and remote-backed projects
- [ ] T010 Adjust `WorkspaceDb::workspace_for_roots_internal(...)` selection to avoid accidentally picking a secondary-window snapshot for “normal open” behavior (prefer Primary when multiple entries share roots) in `crates/workspace/src/persistence.rs`

**Checkpoint**: Secondary windows can serialize to distinct workspace ids; persistence can enumerate and load last-session workspaces by id across local + remote-backed origins.

---

## Phase 3: User Story 1 - Restore Tabs into Correct Windows (Priority: P1) 🎯 MVP

**Goal**: Restore multi-window session so each window’s tabs restore into the same window (no unintended duplication into primary), honoring system window tabs and project-origin boundaries.

**Independent Test**: `specs/001-multi-window-session/quickstart.md` Scenarios A, A2, B.

### Tests for User Story 1 (required) ⚠️

- [ ] T011 [P] [US1] Add persistence test: two workspaces with identical roots but different `workspace_id` + `window_role` are persisted and can be queried distinctly for last-session restore in `crates/workspace/src/persistence.rs`
- [ ] T012 [P] [US1] Add persistence test: last-session listing includes remote-backed workspaces and preserves ordering by window stack in `crates/workspace/src/persistence.rs`
- [ ] T013 [P] [US1] Add GPUI test: create primary + secondary window, open distinct files in each, persist, simulate restore-by-id flow, and assert no unintended duplicates (same file = (project origin, canonical absolute path)) in `crates/workspace/src/workspace.rs` (or `crates/zed/src/main.rs` test module if more appropriate)
- [ ] T014 [P] [US1] Add GPUI test: missing files do not prevent restore; missing item is shown unavailable and other tabs restore in `crates/workspace/src/workspace.rs` (or `crates/zed/src/main.rs` test module if more appropriate)
- [ ] T015 [P] [US1] Add GPUI test: unsaved buffers restore only in their original window (by persisted buffer id) and are not duplicated across windows in `crates/workspace/src/workspace.rs`

### Implementation for User Story 1

- [ ] T016 [US1] Update session restore to restore by persisted `workspace_id` entries (not by `workspace::open_paths` chooser) in `crates/zed/src/main.rs` (`restore_or_create_workspace`, `restorable_workspace_locations`)
- [ ] T017 [US1] Ensure restore honors system window tabs setting (when enabled, restore additional windows as tabs) in `crates/zed/src/main.rs` (preserve the “wait for first window” behavior where required)
- [ ] T018 [US1] Implement “open window from serialized workspace” helper that creates a window with the correct role/id and calls `Workspace::load_workspace(...)` in `crates/workspace/src/workspace.rs` (must support both local and remote-backed workspaces)
- [ ] T019 [US1] Ensure restore does not merge items across windows; prevent unintended duplicates by de-duplicating within a window by (project origin, canonical absolute path) in `crates/workspace/src/workspace.rs`
- [ ] T020 [US1] Preserve tab order and active tab per window during restore in `crates/workspace/src/workspace.rs` (pane/group deserialization + active pane selection)
- [ ] T021 [US1] Preserve window ordering when restoring multiple windows based on stored window stack in `crates/zed/src/main.rs` and `crates/workspace/src/persistence.rs`
- [ ] T022 [US1] Ensure missing-file restore behavior matches FR-007 (unavailable item UI, other tabs still restore) in `crates/workspace/src/workspace.rs`
- [ ] T023 [US1] Ensure unsaved buffers restore only in their original window (by persisted buffer id) and are never duplicated across windows in `crates/workspace/src/workspace.rs`
- [ ] T024 [US1] Implement window-limit handling: partial restore + non-modal toast/notification in the restored primary window summarizing what could not be restored in `crates/zed/src/main.rs` and `crates/workspace/src/toast_layer.rs` (or existing notification mechanism)
- [ ] T025 [US1] Implement remote reconnect failure behavior: if a remote-backed window can’t reconnect, restore it in disconnected state, prompt to reconnect, and restore tabs once connected in `crates/zed/src/main.rs` and `crates/workspace/src/workspace.rs`

**Checkpoint**: US1 complete; session restore produces the correct set of windows/tabs without unintended duplicates, respects system window tabs, and preserves project-origin separation.

---

## Phase 4: User Story 2 - Persist Window/Tab State on Close (Priority: P2)

**Goal**: Closing windows updates persisted state so the next restart reflects the most recent window/tabs configuration (local and remote-backed).

**Independent Test**: `specs/001-multi-window-session/quickstart.md` Scenario C.

### Tests for User Story 2 (required) ⚠️

- [ ] T026 [P] [US2] Add GPUI test: close a secondary window, then verify persisted last-session workspaces no longer include the closed window’s `workspace_id` in `crates/workspace/src/workspace.rs`

### Implementation for User Story 2

- [ ] T027 [US2] Ensure closing a secondary window properly detaches it from the session (clears `session_id` and/or updates DB) without affecting other windows in `crates/workspace/src/workspace.rs` (`Workspace::close_window`, `remove_from_session`, `serialize_workspace_internal`)
- [ ] T028 [US2] Ensure window/tab state is serialized for both primary and secondary windows at close-time (not just periodically) in `crates/workspace/src/workspace.rs`

**Checkpoint**: US2 complete; closed windows do not reappear and their tabs are not merged into primary on next restore.

---

## Phase 5: User Story 3 - Rust Analyzer Workspace Discovery Works After Restore (Priority: P2)

**Goal**: Fix the underlying cause so rust-analyzer can discover the workspace for valid Rust projects after open/restore (no message suppression).

**Independent Test**: `specs/001-multi-window-session/quickstart.md` Scenario D.

### Tests for User Story 3 (required) ⚠️

- [ ] T029 [P] [US3] Add unit test verifying the rust-analyzer initialization parameters include all visible worktree roots (and/or workspace folders) needed to discover `Cargo.toml`-based workspaces in `crates/project/src/lsp_store.rs` (or the module that builds rust-analyzer init options)

### Implementation for User Story 3

- [ ] T030 [US3] Identify where rust-analyzer init options/workspace folders are constructed and why restored/opened projects might omit necessary roots in `crates/project/src/lsp_store.rs`
- [ ] T031 [US3] Implement the fix so rust-analyzer receives correct workspace information after open/restore and no longer reports “Failed to discover workspace” for valid Rust projects in `crates/project/src/lsp_store.rs` (and related rust-analyzer configuration plumbing)

**Checkpoint**: US3 complete; rust-analyzer discovers the workspace successfully after restore for valid Rust projects.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup.

- [ ] T032 Run `specs/001-multi-window-session/quickstart.md` Scenarios A–F manually and record outcomes
- [ ] T033 [P] Run `cargo test -p workspace -p project -p zed` and fix any new failures related to this feature
- [ ] T034 [P] Run `./script/clippy` (or `./script/clippy.ps1` on Windows) and fix any new warnings in modified files
- [ ] T035 Ensure no new `unwrap()` / `expect()` in non-test code paths in modified files

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Blocks all user stories
- **US1 (Phase 3)**: Depends on Phase 2
- **US2/US3 (Phase 4–5)**: Depend on Phase 2; can proceed after (or in parallel with) US1 once Phase 2 is complete
- **Polish (Phase 6)**: After desired user stories are complete

### Parallel Opportunities

- **Phase 1**: T001–T003 can be done in parallel
- **Phase 2**: T004/T005 can proceed with T006/T007 once schema is decided; T008–T010 may overlap but must coordinate on shared files
- **Phase 3**: T011–T015 can be written in parallel (tests), then implementation tasks proceed
- **Phase 5**: T029 can be written in parallel with other story work (separate crate/file)

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 + Phase 2
2. Complete US1 (Phase 3) and validate Scenarios A, A2, B
3. Then proceed to US2 + US3


