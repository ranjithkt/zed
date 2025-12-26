# Research: Multi-Window Session Restore

**Feature**: `001-multi-window-session`  
**Date**: 2025-12-26  
**Status**: Complete

## What’s happening today

### Session restore uses “open paths”

On app start / reopen, Zed restores workspaces in `crates/zed/src/main.rs` by calling `workspace::open_paths(...)` for each entry returned by `restorable_workspace_locations(...)`.

This restore approach is path-based, and the global `open_paths` chooser attempts to find an existing “best match” window for the given paths. That is useful for normal “open folder” behavior, but it is **not suitable** for restoring a multi-window session where multiple windows can share the same roots.

### Persisted workspaces are keyed by roots

When opening a local project (`Workspace::new_local`), the code looks up a serialized workspace using `WorkspaceDb::workspace_for_roots(...)` and reuses that workspace id if found. This design assumes “one serialized workspace per root set”.

In a multi-window world, this assumption breaks: multiple windows can share the same roots but must keep separate tab/pane state.

### Secondary editor windows are not guaranteed to be persisted

Secondary windows are created with `Workspace::new_with_role(None, ..., SecondaryEditor, ...)`. Serialization requires `database_id` to be present; without a workspace id, a secondary window cannot serialize its state. To support multi-window restore, secondary windows must have stable workspace ids that can be saved and later restored.

### rust-analyzer workspace discovery failure

The terminal output you shared:

> Language server rust-analyzer status update: Failed to discover workspace. Consider adding the Cargo.toml of the workspace to linkedProjects...

is produced by rust-analyzer via `experimental/serverStatus` notifications. The desired behavior is **not** to suppress these messages, but to fix the underlying cause so rust-analyzer can successfully discover the workspace for valid Rust projects.

## Root causes

1. **Restore uses a chooser intended for “open folder”** instead of restoring by persisted per-window workspace records.
2. **Persistence does not model “multiple windows for the same roots”** as distinct persisted workspaces with distinct ids and roles.
3. **Restore/open flows can cause rust-analyzer discovery to fail** even when the same project works when opened normally.

## Design direction

- Restore last session by enumerating **workspace ids** that were open in the last session (ordered by the saved window stack), then open windows and load each `SerializedWorkspace` directly.
- Ensure primary and secondary windows have distinct persisted workspace ids so they can serialize independently.
- Fix the underlying cause of rust-analyzer workspace discovery failures triggered by restore/open flows, without suppressing status messages.

## Guiding principle (do not reinvent the wheel)

- Multi-window restore must reuse existing single-window restore behavior for all within-window concerns (duplicates, missing files, unsaved buffers, remote reconnect UX). The only intended change is preventing cross-window collapsing/merging by restoring per-window snapshots.


