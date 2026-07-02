---
name: rust-workflow
description: Format, lint, build, test, and report completion for Rust changes in this repository.
---

# Rust Workflow

## Purpose

Define the standard local validation sequence for changes to the Rust codebase. Use this skill before declaring a code change complete.

## Validation Sequence

Run these commands in order. Stop and fix the first failure before proceeding.

1. Format
   ```sh
   cargo fmt --check
   ```
   If this fails, apply formatting with:
   ```sh
   cargo fmt
   ```

2. Lint
   ```sh
   cargo clippy --all-targets --all-features -- -D warnings
   ```
   Resolve every warning unless the warning is a known false positive. If you must allow a warning, document the reason in code or in the change summary.

3. Build
   ```sh
   cargo build
   ```
   The release profile is not required for ordinary changes. Use `cargo build --release` when validating performance-sensitive or deployment-relevant code.

4. Test
   ```sh
   cargo test
   ```
   If tests are slow or require external services, run the relevant subset and state what was skipped in the completion summary.

## Makefile

The project includes a `Makefile`. You may use `make run` for local development if it matches the current runtime target. Do not rely on `make` as a substitute for the validation sequence above unless a recipe explicitly runs the same commands.

## Completion Report

At the end of a meaningful Rust change, report:

- What changed
- Which files were added, removed, or modified
- Which tests were added or updated
- Which validation commands were run and their results
- Any remaining risks or skipped checks
