# AI Agent Instructions

This repository contains **SAKEM@S bot**, a Rust Discord bot for a single community server. Agents working here should keep changes focused, consistent with the existing Rust style, and aligned with the project-specific skills under `.agents/skills/`.

## Error Handling

Do not fight the same error twice. If an error repeats, stop and identify 3–5 plausible causes before choosing the most efficient fix.

## Skills

Additional project-specific guidance lives under:

`.agents/skills/`

Consult `.agents/skills/INDEX.md` before architectural decisions, design-document changes, or significant refactors.

The most frequently used skills are:

- `rust-workflow` — local validation sequence for Rust changes
- `adr-review` — evaluate major architectural decisions
- `design-sync` — keep `README.md` and `design/` documents synchronized with code
- `document-date-accuracy` — anchor dates and relative-time claims to the OS clock

## Repository Integrity

Preserve encapsulation, separation of concerns, invariants, consistency, and fitness for purpose.

Prefer concrete boundary language such as data ownership, invariant enforcement, API surface, dependency direction, execution flow, and source of truth. Avoid vague umbrella terms when a more testable boundary can be named.

Before introducing a new module, abstraction, dependency, or boundary split, check whether an existing concept already owns the data, invariant, API surface, or execution flow.

Prefer a single source of truth over duplicated knowledge. Do not place the same domain rule, schema, metadata definition, storage invariant, or boundary contract in multiple locations.

Use the `adr-review` skill for changes that affect architecture, storage layout, public APIs, deployment, or runtime behavior.

## Design Documents

Keep `README.md` accurate. If a `design/` directory is created later, treat it as active design contracts, not archival notes.

When a code change affects architecture, storage layout, command semantics, public APIs, migration requirements, or failure modes, update the relevant documentation in the same patch.

Use the `design-sync` skill for changes that may invalidate, refine, or require status updates in documentation.

Use the `adr-review` skill for major architectural decisions.

## Date Accuracy in Documents

When creating, editing, or reviewing documents that include dates, relative time, timelines, deadlines, schedules, milestones, or words such as `today`, `recent`, `latest`, `current`, `now`, `as of`, `last`, or `next`, use the `document-date-accuracy` skill.

Do not rely on model memory for the current date. Use UTC for document time notation.

## Test-First Contract

Tests are first-class boundaries. They must preserve intended domain behavior and cover high-risk areas with sufficient precision.

When behavior changes intentionally, update tests to reflect the new contract. When coverage is insufficient, add tests proactively. Do not weaken tests merely to make an implementation pass.

## Rust Workflow

After completing a meaningful Rust change, run the local validation sequence described in the `rust-workflow` skill.

The sequence is:

1. `cargo fmt --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo build`
4. `cargo test`

Fix the first failure before proceeding to the next step.

## Expected Completion Summary

At the end of a meaningful change, report:

- What changed
- Which skills were used, if any
- Which files were added, removed, or modified
- Which tests were added or updated
- Which design documents were checked or updated
- Which format, lint, build, and test commands were run
- Any remaining risks or skipped checks
