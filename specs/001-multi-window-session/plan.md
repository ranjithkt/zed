# Implementation Plan: Multi-Window Session Restore

**Branch**: `001-multi-window-session` | **Date**: 2025-12-26 | **Spec**: `spec.md`  
**Input**: Feature specification from `specs/001-multi-window-session/spec.md`

## Summary

Restore multi-window sessions by loading and restoring **persisted workspace ids per window** (not by path-based “open” heuristics), ensuring:

- Each previously-open window’s tabs/panes restore into the same window (including when represented as system window tabs).
- No unintended duplicates are created within a window (de-dup key: **(project origin, canonical absolute path)**).
- Remote-backed sessions (WSL/SSH/remote server) restore within their origin; reconnect failures restore disconnected and prompt to reconnect.
- rust-analyzer workspace discovery succeeds after open/restore by initializing with **workspace folders for all visible worktree roots** in the window.

## Technical Context

**Language/Version**: Rust 1.92 (`rust-toolchain.toml`)  
**Primary Dependencies**: GPUI (Zed UI framework), Zed crates (`workspace`, `project`, `session`, `zed`), async tasks/executors used by GPUI  
**Storage**: Existing workspace persistence (workspace DB / `workspaces` table and serialized workspace blobs)  
**Testing**: `cargo test` (including GPUI tests where applicable); follow GPUI timer guidance for deterministic tests  
**Target Platform**: Zed desktop (macOS/Windows/Linux); macOS may use system window tabs  
**Project Type**: Rust multi-crate workspace (existing Zed repo structure)  
**Performance Goals**: Keep startup/restore responsive; avoid blocking the UI thread on slow IO/remote reconnection  
**Constraints**:
- Avoid panics in non-test code (`unwrap`/`expect` forbidden); propagate errors (`?`) and surface actionable UI errors where appropriate.
- Preserve entity update safety rules (no re-entrant entity updates; use closure `cx`).
- Do not suppress rust-analyzer messages; fix the underlying failure condition.
**Scale/Scope**: Multiple windows per project origin; many tabs per window; “platform window limit” is handled with partial restore and a non-modal notification

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Narrow scope**: Focused on session restore correctness + rust-analyzer discovery for restored/opened projects.
- **No panic control flow**: Plan requires error propagation/logging patterns, not `unwrap`/`expect`.
- **Tests required**: Tests are included for restore and rust-analyzer init parameter construction; manual quickstart scenarios cover platform/system-tab and remote cases.
- **Prefer existing structure**: No new crates; changes scoped to existing crates and persistence model.

**Result**: PASS (no constitution violations required).

## Project Structure

### Documentation (this feature)

```text
specs/001-multi-window-session/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── session-restore.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── zed/
│   └── src/
│       └── main.rs
├── workspace/
│   └── src/
│       ├── workspace.rs
│       ├── persistence.rs
│       └── persistence/
│           └── model.rs
└── project/
    └── src/
        ├── lsp_store.rs
        └── lsp_store/
            └── rust_analyzer_ext.rs
```

**Structure Decision**: Keep implementation within existing Zed crates (`zed`, `workspace`, `project`). Persist additional workspace metadata (role/origin identifiers) and switch restore from path-based open to restore-by-workspace-id.

## Implementation Phases (high level)

### Phase 0: Confirm current behavior (read-only)

- Verify restore entrypoint uses path-based `open_paths` behavior and collapses multiple windows.
- Verify secondary editor windows lack a persisted `WorkspaceId` and therefore can’t serialize.
- Capture rust-analyzer “Failed to discover workspace” reproduction conditions.

### Phase 1: Persistence + restore primitives

- Persist enough metadata to uniquely restore windows:
  - `workspace_id`, `session_id`, `window_id` (ordering only), `window_role`, `location/origin`, `paths`.
- Add APIs to enumerate last-session workspaces in session window stack order and load by `workspace_id`.
- Ensure secondary windows are created with persisted ids and serialize independently.

### Phase 2: Restore-by-id + correctness

- Restore session by enumerated workspace ids:
  - Create windows (or system tabs) per session entry.
  - Load serialized workspaces into the created windows without cross-window merging.
  - De-duplicate within a window by **(project origin, canonical absolute path)** only.
- Remote restore:
  - Reconnect per origin; on failure restore disconnected and prompt; restore tabs once connected.
- Platform window limits:
  - Partial restore + non-modal notification/toast in the primary window.

### Phase 3: rust-analyzer workspace discovery fix

- Ensure rust-analyzer initialization includes workspace folders for **all visible worktree roots** in the window, so Cargo workspace discovery succeeds after restore/open.

### Phase 4: Tests + validation

- Add/adjust tests to cover:
  - Persistence and last-session enumeration.
  - Restore-by-id preventing unintended duplicates.
  - rust-analyzer init parameter construction (workspace folders).
- Run `quickstart.md` scenarios A–F across local + remote + system-tab setting.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
