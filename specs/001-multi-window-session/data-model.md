# Data Model: Multi-Window Session Restore

## Overview

This feature requires persisting and restoring **multiple windows** for the same project roots, with each window maintaining its own tab/pane state.

**Guiding principle (do not reinvent the wheel)**: This data model only describes what must exist to restore multiple windows independently. It MUST NOT introduce new semantics for within-window restore behavior (duplicates, missing files, unsaved buffers), which remains whatever Zed already does for single-window restore.

## Entities

### Session Window State

Represents the persisted “set of windows that were open” for a session and their ordering.

- **session_id**: Identifier for the session being restored.
- **window_stack**: Ordered list of window ids from the prior session (front-to-back).

### Serialized Workspace

Represents a persisted workspace snapshot for a single window.

- **workspace_id**: Stable identifier for the workspace snapshot (used as a persistence key).
- **session_id**: The session that the workspace belongs to when it was last serialized.
- **window_id**: The runtime window id at the time of serialization (used for ordering only).
- **window_role**: Primary or SecondaryEditor (needed to select the correct “default” workspace for a root set).
- **location**: Local or Remote connection metadata.
- **paths**: Root paths for the project/worktrees associated with this window.
- **center_group**: Serialized pane split tree + serialized items.
- **docks**: Serialized dock state (left/right/bottom).
- **window_bounds / display**: Saved placement and bounds for restore.

### Editor Item / Tab

Represents an open editor item within a specific serialized workspace’s pane tree.

- **project_path (optional)**: Used by existing single-window restore behavior to match already-restored items and avoid reopening duplicates in the same window.
- **order / active**: Preserves tab ordering and selection within each pane.

## Relationships

- A **Session Window State** references many **Serialized Workspaces** via their `window_id` ordering and shared `session_id`.
- A **Serialized Workspace** contains many **Editor Items/Tabs**, nested inside a pane tree.
- Multiple **Serialized Workspaces** may share the same `paths` (same project roots) but must remain distinct by `workspace_id` and `window_role`.


