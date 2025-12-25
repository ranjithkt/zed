<!--
Sync Impact Report
- Version change: TEMPLATE (unversioned) → 1.0.0
- Modified principles: N/A (initial ratification)
- Added sections:
  - Filled all placeholders in this constitution
  - Zed-specific principles for Rust + GPUI development
  - Engineering Constraints
  - Workflow & Quality Gates
  - Governance (amendment + versioning policy)
- Removed sections: None
- Templates requiring updates:
  - ✅ .specify/templates/plan-template.md (fix non-existent commands path reference)
  - ✅ .specify/templates/tasks-template.md (align test guidance + add Rust workspace paths)
  - ✅ .cursor/commands/speckit.constitution.md (fix commands directory reference)
  - ✅ .cursor/commands/speckit.tasks.md (align test guidance with constitution)
- Follow-up TODOs: None
-->
# Zed Constitution

## Core Principles

### Ship small, desired changes
- Changes MUST be narrowly scoped and easy to review.
- Large refactors MUST be avoided unless explicitly requested and justified.
- Features SHOULD be confirmed as desired before significant implementation work begins.
- The change set MUST be “about one thing” (avoid mixing feature work, refactors, and drive-by edits).

### Rust: correctness, explicit errors, and panic avoidance
- Code correctness and clarity MUST be prioritized over cleverness or micro-optimizations unless
  explicitly requested.
- `unwrap()`, `expect()`, and other panic-based control flow MUST be avoided in non-test code.
- Fallible operations MUST not have their errors silently discarded. Use `?` to propagate errors, or
  handle errors explicitly, or use a logging helper (e.g. `.log_err()`) when intentionally ignoring.
- Indexing and other operations that can panic MUST be written defensively (bounds checks, safe APIs).
- Async operations that may fail MUST propagate meaningful errors up to the UI layer so users get
  actionable feedback.

### GPUI: follow current APIs and entity safety rules
- Code MUST use current GPUI APIs and types:
  - Use `Entity<T>` (not `Model<T>` / `View<T>`).
  - Use `App` (not `AppContext`) and `Context<T>` (not `ModelContext<T>`).
  - Use the new async closure forms for `spawn` (e.g. `cx.spawn(async move |cx| ...)`).
- Entity updates MUST avoid re-entrant updates (updating an entity while it is already being updated).
- Within `read_with` / `update` / `update_in` closures, the closure’s `cx` MUST be used (not the outer
  `cx`) to avoid borrow issues and subtle bugs.
- Background work MUST be managed deliberately:
  - If a `Task` must run to completion, it MUST be awaited, detached, or stored to avoid cancellation.

### Tests: required for behavior changes, deterministic by default
- Changes that add or modify behavior MUST include tests unless there is a clear, documented reason
  tests are not feasible.
- Tests MUST be deterministic and avoid timing flakiness.
- In GPUI tests, timers/delays/timeouts MUST use GPUI executor timers (e.g.
  `cx.background_executor().timer(duration).await`) instead of `smol::Timer::after(...)` when driving
  `run_until_parked()` or relying on GPUI scheduling.

### Documentation and repo hygiene are part of “done”
- Documentation changes in `docs/src/` MUST follow `docs/AGENTS.md` (mdBook structure, Prettier
  formatting, anchors, and scope constraints).
- Licensing compliance MUST be maintained; if CI/license checks fail, follow the guidance in
  `README.md` (e.g. `cargo-about` workflow and accepted licenses list).
- Prefer existing files and existing crates; new files/crates MUST have a clear purpose.
- `mod.rs` paths MUST be avoided for new modules (prefer `src/some_module.rs`).
- When creating new crates, prefer specifying the library root path in `Cargo.toml` using
  `[lib] path = "...rs"` instead of the default `lib.rs`.
- Prefer full, descriptive names (no single-letter abbreviations); use variable shadowing to scope
  clones in async contexts to minimize borrow lifetimes.

## Engineering Constraints

- The default implementation language is Rust, and changes MUST adhere to the repo’s Rust + GPUI
  conventions.
- When linting is needed, prefer repo scripts (e.g. `./script/clippy`) over ad-hoc equivalents.
- Avoid creative additions and “nice-to-haves” unless explicitly requested or required to satisfy the
  stated change.
- Comments SHOULD explain “why” something is non-obvious; avoid commentary that restates the code.

## Workflow & Quality Gates

- Before implementation, work MUST be consistent with the current constitution and the repository’s
  contribution norms (`CONTRIBUTING.md`).
- Before shipping:
  - Code changes MUST compile and tests MUST pass for the affected area.
  - Linting SHOULD be run when touching non-trivial Rust code (prefer `./script/clippy`).
  - If `docs/src/` is changed, Prettier MUST pass per `docs/AGENTS.md`.
- If a guideline must be violated for a concrete reason, that violation MUST be explicitly documented
  in the relevant plan/PR context, with the simplest rejected alternative recorded.

## Governance

- This constitution is the highest-level project guidance for Speckit-generated artifacts and
  assistant-driven changes.
- All changes to `.specify/memory/constitution.md` MUST:
  - Include an updated Sync Impact Report at the top of the file.
  - Update dependent templates/command docs to remain consistent.
  - Follow semantic versioning:
    - **MAJOR**: Breaking governance changes, principle removals, or redefinitions.
    - **MINOR**: New principle/section or materially expanded guidance.
    - **PATCH**: Clarifications, wording fixes, non-semantic refinements.
- Reviews MUST treat constitution violations as blockers unless the constitution itself is amended.

**Version**: 1.0.0 | **Ratified**: 2025-12-24 | **Last Amended**: 2025-12-24
