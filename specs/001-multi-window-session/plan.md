# Implementation Plan: Multi-Window Session Restore

**Branch**: `001-multi-window-session` | **Date**: 2025-12-26 | **Spec**: [spec.md](spec.md)  
**Goal**: Restore multi-window sessions without duplicating secondary-window tabs into the primary window (including system window tabs when enabled), and fix rust-analyzer workspace discovery failures triggered by open/restore flows.

## Summary

This feature has two coupled problems:

1. **Session restore collapses multi-window state**: On startup/reopen, restore currently opens workspaces using `workspace::open_paths(...)` based on root paths. That path-selection logic is designed to choose a “best” existing window, which can cause multiple session workspaces (especially those sharing the same roots) to restore into the same window, leading to duplicated tabs and lost window separation.
2. **rust-analyzer workspace discovery failures**: rust-analyzer reports “Failed to discover workspace” during open/restore. The desired outcome is to fix the underlying cause so workspace discovery succeeds for valid Rust projects.

The plan is to:

- Restore **by persisted workspace records** (per window, per project origin) rather than “open paths and let the chooser decide”.
- Ensure **secondary editor windows have their own persistent workspace ids** so their tabs can be saved and restored.
- Ensure rust-analyzer workspace discovery succeeds for valid Rust projects after open/restore (no message suppression).

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

### 3) Fix rust-analyzer workspace discovery on open/restore

- Identify why rust-analyzer cannot discover the workspace in restored/opened projects.
- Ensure Zed provides rust-analyzer with sufficient workspace root information so discovery succeeds without manual configuration.

## Files Expected to Change

- `crates/zed/src/main.rs` (startup restore flow)
- `crates/workspace/src/workspace.rs` (secondary window creation, restore helpers)
- `crates/workspace/src/persistence.rs` (queries for session restore; workspace role persistence)
- `crates/workspace/src/persistence/model.rs` (serialized model updates)
- `crates/session/src/session.rs` (ordering/stack is already persisted; likely no change)
- `crates/project/src/lsp_store/rust_analyzer_ext.rs` (diagnostics/telemetry around discovery failures; verify fix)

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

### Phase 4: Rust Analyzer workspace discovery fix

- Fix the underlying cause of rust-analyzer “failed to discover workspace” during open/restore.
- Validate with manual scenario and (where feasible) tests.

### Phase 5: Tests + Manual Verification

- Add GPUI tests for:
  - Multi-window persistence: primary+secondary both persist and restore without duplication.
  - Session restore ordering: window stack ordering respected (where possible).
  - rust-analyzer workspace discovery: valid Rust workspaces are discoverable after open/restore (no “Failed to discover workspace”).

Manual verification steps: [quickstart.md](quickstart.md)

## Next Step

Run `/speckit.tasks` to break this plan into concrete, ordered implementation tasks.


