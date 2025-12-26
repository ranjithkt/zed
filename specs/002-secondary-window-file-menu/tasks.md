# Tasks: Secondary Window File Menu Integration

**Input**: Design documents from `/specs/002-secondary-window-file-menu/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/action-routing-window-context.md

**Tests**: Tests are included for behavior changes per constitution requirements.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Rust workspace (Zed)**: `crates/<crate_name>/src/` and tests inline or in `crates/<crate_name>/tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: No new project structure needed; this feature modifies existing code only.

- [ ] T001 Review existing `WorkspaceStore::active_editor_window_for_project` API in `crates/workspace/src/workspace.rs` to confirm it tracks pane focus across windows

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core File menu dispatch routing that MUST be complete before ANY user story can be independently verified.

**⚠️ CRITICAL**: This phase implements the contract from `contracts/action-routing-window-context.md`. All user stories depend on this routing infrastructure.

### Implementation

- [ ] T002 Add helper method `resolve_file_menu_target_window` to `ApplicationMenu` in `crates/title_bar/src/application_menu.rs` that computes the target workspace window for File menu dispatch using `WorkspaceStore::active_editor_window_for_project`
- [ ] T003 Modify `build_menu_from_items` in `crates/title_bar/src/application_menu.rs` to use target window dispatch for File menu entries (check `entry.menu.name == "File"`)
- [ ] T004 Add fallback logic: if `active_editor_window` is missing or stale, fall back to dispatching in the primary (current) window as per contract
- [ ] T005 Ensure errors resolving target workspace are logged (not panicked or silently ignored) per constitution

**Checkpoint**: File menu actions now dispatch to the correct editor window. Manual verification: focus secondary window editor, click File > Save from primary menu, observe action targets secondary.

---

## Phase 3: User Story 1 - Save File in Secondary Window (Priority: P1) 🎯 MVP

**Goal**: File > Save, Save As, and Save All operate on the active editor window (including secondary windows).

**Independent Test**: Open secondary window, create unsaved file, focus its editor, trigger File > Save from primary menu; file in secondary should be saved.

### Tests for User Story 1 ⚠️

- [ ] T006 [P] [US1] Add GPUI test: dispatch `Save` action via application menu when secondary window has pane focus; assert save targets secondary workspace in `crates/title_bar/tests/application_menu_routing.rs` (create file if needed, or add to existing workspace tests in `crates/workspace/src/workspace.rs`)
- [ ] T007 [P] [US1] Add GPUI test: regression check that `Save` still targets primary workspace when primary has focus
- [ ] T007a [P] [US1] Add GPUI test: verify keyboard shortcut (Ctrl+S / Cmd+S) dispatches to focused window when secondary has pane focus (covers FR-012)

### Implementation for User Story 1

- [ ] T008 [US1] Verify `workspace::Save` action handler in `crates/workspace/src/workspace.rs` uses its own window context (already correct; confirm no hardcoded primary assumption)
- [ ] T009 [US1] Verify `workspace::SaveAs` action handler shows dialog on the **origin window** (where File menu was clicked), not the target editor window; this keeps dialogs near the user's mouse/attention per FR-013
- [ ] T010 [US1] Verify `workspace::SaveAll` handler in `crates/workspace/src/workspace.rs`; per spec FR-011 it should save across all project windows (may need iteration over window group)
- [ ] T011 [US1] If `SaveAll` only saves current workspace, update it to iterate `WorkspaceStore::secondary_windows_for_project` and save in each, plus primary

**Checkpoint**: User Story 1 complete. Manual test per `quickstart.md` Scenario A/B/C/E.

---

## Phase 4: User Story 2 - File Creation and Opening in Secondary Window (Priority: P1)

**Goal**: File > New and File > Open open files in the active editor window, not always the primary.

**Independent Test**: Focus secondary window, trigger File > New; new untitled buffer appears in secondary, not primary.

### Tests for User Story 2 ⚠️

- [ ] T012 [P] [US2] Add GPUI test: dispatch `NewFile` action via application menu when secondary window has pane focus; assert new buffer appears in secondary workspace in test file from T006/T007

### Implementation for User Story 2

- [ ] T013 [US2] Audit `workspace::NewFile` handler registration in `crates/zed/src/zed.rs` (`initialize_workspace`); currently calls `open_new()` which creates a new window—change to open in current workspace when invoked via File menu dispatch
- [ ] T014 [US2] Refactor `NewFile` handler: when action is dispatched into a workspace, use `Editor::new_file(workspace, ...)` on that workspace instead of `open_new(...)`
- [ ] T015 [US2] Verify `workspace::Open` / `workspace::OpenFiles` handlers in `crates/workspace/src/workspace.rs` respect the window they're dispatched into (confirm file picker + open happen in target window)
- [ ] T016 [US2] Verify `Reopen Last Closed` (`workspace::ReopenClosedItem`) uses the receiving workspace's closed-item history, not a global or primary-only history

**Checkpoint**: User Story 2 complete. Manual test: focus secondary, File > New opens there; File > Open opens selected file in secondary.

---

## Phase 5: User Story 3 - File Closure in Secondary Window (Priority: P2)

**Goal**: File > Close Editor and File > Close All close tabs in the active editor window only.

**Independent Test**: Open files in both windows, focus secondary, trigger File > Close Editor; only secondary's active tab closes.

### Tests for User Story 3 ⚠️

- [ ] T017 [P] [US3] Add GPUI test: dispatch `CloseActiveItem` via application menu when secondary has focus; assert only secondary workspace's item closes

### Implementation for User Story 3

- [ ] T018 [US3] Verify `workspace::CloseActiveItem` handler (File > Close Editor) in `crates/workspace/src/workspace.rs` operates on its own workspace (should already be correct as it's workspace-scoped)
- [ ] T018a [US3] Verify `workspace::CloseAllItems` handler (File > Close All) in `crates/workspace/src/workspace.rs` closes only items in the receiving workspace, not all windows
- [ ] T019 [US3] Verify `workspace::CloseWindow` targets the receiving workspace's window (should be correct; confirm)
- [ ] T020 [US3] Confirm unsaved-changes prompt dialog appears attached to the **origin window** (where File menu was clicked), not the target editor window, when closing with unsaved changes

**Checkpoint**: User Story 3 complete. Manual test per `quickstart.md` Scenario D.

---

## Phase 6: User Story 4 - File Properties and Metadata Operations (Priority: P3)

**Goal**: Copy Path, Reveal in Explorer, and similar metadata operations use the active file in the target editor window.

**Independent Test**: Open different files in primary and secondary, focus secondary, trigger File > Copy Path; clipboard contains path of secondary's active file.

### Tests for User Story 4 ⚠️

- [ ] T021 [P] [US4] Add GPUI test: dispatch Copy Path action when secondary has focus; assert clipboard contains path of file in secondary workspace

### Implementation for User Story 4

- [ ] T022 [US4] Verify Copy Path action in `crates/editor/src/editor.rs` (search for `CopyPath` action handler) uses the receiving workspace's active item path
- [ ] T023 [US4] Verify Reveal in Explorer action in `crates/editor/src/editor.rs` or `crates/workspace/src/workspace.rs` (search for `RevealInFileManager` or similar) uses the receiving workspace's active item path
- [ ] T024 [US4] Audit remaining File menu metadata actions defined in `crates/zed/src/zed/app_menus.rs` for correct window targeting

**Checkpoint**: User Story 4 complete. All File menu metadata operations respect active editor window.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup.

- [ ] T025 Run `quickstart.md` full manual verification across Windows, macOS (with cross-platform menu), and Linux
- [ ] T026 [P] Run `./script/clippy` and fix any new warnings in modified files
- [ ] T027 [P] Ensure no `unwrap()` / `expect()` in non-test code paths per constitution
- [ ] T028 Review error handling: verify fallback to primary window logs a warning when active editor window lookup fails
- [ ] T029 Update `quickstart.md` if any scenarios need adjustment based on implementation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - review only
- **Foundational (Phase 2)**: Depends on Setup review - **BLOCKS all user stories**
- **User Stories (Phase 3–6)**: All depend on Foundational phase (dispatch routing) completion
  - US1 and US2 are both P1; can proceed in parallel after Foundational
  - US3 (P2) and US4 (P3) can proceed after or in parallel with P1 stories
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends only on Foundational phase (Phase 2)
- **User Story 2 (P1)**: Depends only on Foundational phase (Phase 2); can run in parallel with US1
- **User Story 3 (P2)**: Depends only on Foundational phase; can run in parallel with US1/US2
- **User Story 4 (P3)**: Depends only on Foundational phase; can run in parallel with others

### Within Each User Story

- Tests written and confirmed to fail before implementation (when applicable)
- Verify existing handlers before modifying
- Core implementation before integration
- Story complete and manually verified before moving to next

### Parallel Opportunities

- **Phase 2**: T002–T005 are sequential (building on each other)
- **Phase 3–6**: All test tasks (T006, T007, T007a, T012, T017, T021) marked [P] can be written in parallel once Foundational is done
- **User stories**: US1 and US2 (both P1) can be worked in parallel after Phase 2
- **Polish**: T026 and T027 can run in parallel

---

## Parallel Example: After Foundational Phase

```bash
# Developer A: User Story 1 (Save operations)
T006, T007, T007a (tests) → T008, T009, T010, T011

# Developer B: User Story 2 (New/Open operations)
T012 (test) → T013, T014, T015, T016

# Developer C: User Story 3 (Close operations)
T017 (test) → T018, T018a, T019, T020
```

---

## Implementation Strategy

### MVP First (Foundational + User Story 1)

1. Complete Phase 1: Setup (review)
2. Complete Phase 2: Foundational dispatch routing (**CRITICAL**)
3. Complete Phase 3: User Story 1 (Save operations)
4. **STOP and VALIDATE**: Test US1 per quickstart Scenario A/B/C/E
5. Demo/deploy if ready

### Incremental Delivery

1. Foundational → US1 (MVP: save works in secondary)
2. Add US2 → Test independently (New/Open work in secondary)
3. Add US3 → Test independently (Close works in secondary)
4. Add US4 → Test independently (Metadata operations work)
5. Polish → Full verification

---

## Notes

- [P] tasks = different files or no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable after Foundational phase
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Key insight: most action handlers are already workspace-scoped; the fix is in **dispatch target selection**, not handler logic

### Terminology Mapping (Spec ↔ Code)

| Spec Term | Code Action |
|-----------|-------------|
| File > Close Editor | `workspace::CloseActiveItem` |
| File > Close All | `workspace::CloseAllItems` or `workspace::CloseInactiveItems` |
| File > Save | `workspace::Save` |
| File > Save As | `workspace::SaveAs` |
| File > Save All | `workspace::SaveAll` |
| File > New | `workspace::NewFile` |
| File > Open | `workspace::Open` / `workspace::OpenFiles` |
| File > Reopen Last Closed | `workspace::ReopenClosedItem` |
| File > Close Window | `workspace::CloseWindow` |

