# Contract: Project Window Group Manager (MVP)

This document defines the internal contract for tracking window roles, grouping, and active-editor routing for a single project.

## Responsibilities

- Track which `Workspace` window is **primary** for a project.
- Track which windows are **secondary editor** windows for that project.
- Track the **active editor window** (last window to have editor/pane focus).
- Provide a way to find the target window for project-tree file opens.
- Support primary-close semantics: close all secondary windows, then close primary.

## Inputs

- **Workspace created**: `(project_key, window_id, role)`
- **Workspace destroyed**: `(project_key, window_id)`
- **Pane focused**: `(project_key, window_id)`
- **Primary close requested**: `(project_key, primary_window_id)`

## Outputs / Queries

- `active_editor_window(project_key) -> Option<WindowId>`
- `primary_window(project_key) -> Option<WindowId>`
- `secondary_windows(project_key) -> Vec<WindowId>`
- `target_window_for_project_tree_open(project_key) -> Option<WindowId>`

## Invariants

- Each project has **exactly one** primary window while it is open.
- The active editor window always refers to a window that exists in the group.
- Secondary windows are editor-only and never become “primary”.


