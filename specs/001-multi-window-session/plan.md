# Implementation Plan: Multi-Window Session Restore

**Branch**: `001-multi-window-session` | **Date**: 2025-12-26 | **Spec**: `C:\\Repos\\zed\\specs\\001-multi-window-session\\spec.md`  
**Input**: Feature specification from `C:\\Repos\\zed\\specs\\001-multi-window-session\\spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.cursor/commands/speckit.plan.md` for the execution workflow.

## Summary

Ensure Zed restores multi-window sessions without collapsing multiple windows into the primary window. All within-window restore behavior (duplicates, missing files, unsaved buffers, remote reconnect UX) must remain exactly the same as existing single-window restore behavior.

**Guiding principle (do not reinvent the wheel)**: If a behavior already exists for single-window restore, keep it unchanged. Only add the minimal persistence + restore plumbing needed to save/restore multiple windows without collapsing them.

## Technical Context

**Language/Version**: Rust 1.92 (`C:\\Repos\\zed\\rust-toolchain.toml`)  
**Primary Dependencies**: GPUI and existing Zed crates (`workspace`, `project`, `session`, `zed`)  
**Storage**: Existing workspace persistence (workspace DB / `workspaces` table + serialized workspace blobs)  
**Testing**: `cargo test` (including GPUI tests as appropriate; use GPUI executor timers in tests)  
**Target Platform**: Zed desktop (macOS/Windows/Linux)  
**Project Type**: Rust multi-crate workspace  
**Performance Goals**: Keep restore responsive; avoid long UI stalls from restore bookkeeping  
**Constraints**:
- Must avoid panics in non-test code (`unwrap`/`expect` forbidden).
- Must not silently discard errors; propagate (`?`) or log intentionally.
- Must not change within-window restore behavior relative to single-window restore.
**Scale/Scope**: Multiple windows per project origin; many tabs per window; system window tabs honored when enabled

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Narrow scope: only prevent cross-window collapsing/merging during restore, plus fix rust language discovery regressions caused by restore.
- No panic control flow in non-test code.
- Tests required for behavior changes (prefer deterministic GPUI tests where feasible).
- Prefer existing structure/files; avoid creating new crates or large refactors.

**Result**: PASS

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

**Structure Decision**: Keep changes within existing crates and persistence; add only the minimal new persistence and restore plumbing needed to prevent cross-window collapsing.

## Implementation Phases (high level)

### Phase 0: Confirm current behavior (read-only)

- Confirm restore currently uses path-based “open” logic that can select an existing “best match” window and collapse multiple windows into one.
- Confirm which persisted identifiers exist today and why secondary windows may not serialize distinctly.
- Reproduce rust-analyzer “Failed to discover workspace” in a restore/open flow where the same project works when opened normally.

### Phase 1: Minimal persistence support for multiple windows

- Ensure each window that should restore independently has its own persisted workspace id/snapshot so restore can address windows individually.
- Preserve the existing single-window snapshot selection behavior for normal open flows (avoid changing how a single window is chosen).

### Phase 2: Restore-by-snapshot (multi-window only)

- Restore session by enumerating the saved set of workspace snapshots from the last session and creating one window/tab per snapshot (honoring system window tabs setting).
- Do not introduce new duplicate/missing-file/unsaved-buffer rules; use existing per-window restore behavior.

### Phase 3: Rust language feature regression fix

- Fix the underlying cause of restore/open flows causing rust-analyzer to fail discovery in projects that work when opened normally.
- Do not suppress status messages; eliminate the failure condition.

### Phase 4: Tests + manual verification

- Add deterministic tests for “no collapsing/merging” behavior.
- Add regression test that restore does not introduce the rust-analyzer failure in the covered scenarios (as feasible).
- Run `C:\\Repos\\zed\\specs\\001-multi-window-session\\quickstart.md` scenarios.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
