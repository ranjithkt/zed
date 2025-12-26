# Implementation Plan: Multi-Window Session Restore

**Branch**: `001-multi-window-session` | **Date**: 2025-12-26 | **Spec**: [spec.md](spec.md)  
**Goal**: Restore multi-window sessions without duplicating secondary-window tabs into the primary window, and suppress repeated rust-analyzer “failed to discover workspace” status spam on window open.

## Summary

This feature has two coupled problems:

1. **Session restore collapses multi-window state**: On startup/reopen, restore currently opens workspaces using `workspace::open_paths(...)` based on root paths. That path-selection logic is designed to choose a “best” existing window, which can cause multiple session workspaces (especially those sharing the same roots) to restore into the same window, leading to duplicated tabs and lost window separation.
2. **Repeated rust-analyzer error spam**: rust-analyzer emits repeated `experimental/serverStatus` notifications when workspace discovery fails; we currently log each update verbatim, which spams the terminal/log output during startup and restore.

The plan is to:

- Restore **by persisted workspace records** (per window) rather than “open paths and let the chooser decide”.
- Ensure **secondary editor windows have their own persistent workspace ids** so their tabs can be saved and restored.
- Add **status update deduping** for rust-analyzer server-status notifications (log + UI update throttling), and promote a single actionable message.

## Technical Context

- **Language**: Rust
- **UI framework**: GPUI
- **Session restore entrypoint**: `crates/zed/src/main.rs` (`restore_or_create_workspace`)
- **Persistence**: `crates/workspace/src/persistence.rs` and the `workspaces` table
- **Rust Analyzer status notifications**: `crates/project/src/lsp_store/rust_analyzer_ext.rs`

## Key Code Facts (current behavior)

- Startup restore calls `workspace::open_paths(&paths.paths(), OpenOptions::default(), ...)` per session entry. This uses the “best match” window chooser and can collapse multiple intended windows into one.
- Workspace persistence uses a **single `SerializedWorkspace` per `workspace_id`**, which is currently selected by **roots** via `WorkspaceDb::workspace_for_roots(...)`. This API returns only one match for a root set.
- Secondary editor windows are opened via `Workspace::new_with_role(None, ..., SecondaryEditor, ...)`, which means they have **no `database_id`**, so they cannot serialize state.
- rust-analyzer status updates are logged unconditionally in `rust_analyzer_ext::register_notifications(...)` for each `experimental/serverStatus` notification.

## Proposed Architecture Changes

### 1) Persist per-window workspace state

- Ensure each secondary editor window is created with a fresh `WorkspaceId` (database id), so it can serialize independently.
- Persist the window role (Primary / SecondaryEditor) alongside the serialized workspace, so “open project” behavior can still prefer the primary layout.

### 2) Restore session by workspace ids (not by roots)

- Extend persistence to return the *set of workspaces* that were open in the last session as **workspace ids**, ordered by the stored window stack.
- Add a new restore path that opens windows from these serialized workspaces directly (bounds, docks, panes, tabs), avoiding the global `open_paths` chooser.

### 3) Deduplicate rust-analyzer “serverStatus” spam

- Deduplicate (or rate-limit) repeated identical status messages per `(LanguageServerId, health)` for a time window during startup.
- Still emit one user-visible notification when workspace discovery fails, with actionable guidance, but avoid repeated log spam.

## Files Expected to Change

- `crates/zed/src/main.rs` (startup restore flow)
- `crates/workspace/src/workspace.rs` (secondary window creation, restore helpers)
- `crates/workspace/src/persistence.rs` (queries for session restore; workspace role persistence)
- `crates/workspace/src/persistence/model.rs` (serialized model updates)
- `crates/session/src/session.rs` (ordering/stack is already persisted; likely no change)
- `crates/project/src/lsp_store/rust_analyzer_ext.rs` (dedupe serverStatus logging / UI updates)

## Implementation Phases

### Phase 0: Research & Confirmation

- Confirm how secondary windows should be identified/created on restore (window count, ordering).
- Confirm current persistence schema and whether it can store per-window role safely.

Artifacts: [research.md](research.md)

### Phase 1: Data model + Contract

- Define the persisted “session window state” model and how it maps to runtime windows.
- Define restore contract: ordering, duplication rules, missing file behavior, and error handling.

Artifacts:
- [data-model.md](data-model.md)
- [contracts/session-restore.md](contracts/session-restore.md)

### Phase 2: Session restore plumbing

- Add persistence API to list last-session workspaces as workspace ids ordered by session window stack.
- Implement restore code path to open windows from serialized workspaces and restore items per window.

### Phase 3: Persist secondary windows

- Ensure secondary windows get a database id and serialize state.
- Ensure “open project normally” still selects the appropriate primary workspace state.

### Phase 4: Rust Analyzer status dedupe

- Deduplicate repeated rust-analyzer server status log messages.
- Promote a single actionable user-visible message for discovery failures.

### Phase 5: Tests + Manual Verification

- Add GPUI tests for:
  - Multi-window persistence: primary+secondary both persist and restore without duplication.
  - Session restore ordering: window stack ordering respected (where possible).
  - rust-analyzer spam suppression: repeated identical messages are not logged/emitted repeatedly.

Manual verification steps: [quickstart.md](quickstart.md)

## Next Step

Run `/speckit.tasks` to break this plan into concrete, ordered implementation tasks.


