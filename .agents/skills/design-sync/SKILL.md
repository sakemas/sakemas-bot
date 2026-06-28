---
name: design-sync
description: Keep README.md and design/ documents synchronized with implementation and explicit about status.
---

# Design Sync

## Purpose

Ensure that design documents and the project README remain accurate as the code changes. A document that describes planned behavior must be distinguishable from one that describes implemented behavior.

## When to Update

Update design documentation in the same change when a code change affects:

- Architecture or module boundaries
- Database schema or persistence format
- Public command surface or API
- Deployment or runtime behavior
- Environment variables or secrets
- Observability or failure modes

## Status Markers

Use explicit markers so readers can tell what is current:

- `[Implemented]` — behavior matches the code
- `[In Progress]` — implementation has started but is not complete
- `[Planned]` — agreed direction, no implementation yet
- `[Deprecated]` — replaced by a newer approach; keep until migration is complete

Do not remove a deprecated document until the migration it describes is finished.

## Single Source of Truth

Prefer to record a fact once. If a concept is documented in both `README.md` and `design/`, pick one authoritative location and reference it from the other. Do not duplicate tables, schemas, or environment-variable lists.

## Required Checks

Before finishing a change that touches design-sensitive code:

- Read the relevant `README.md` section
- Read the relevant `design/` document, if one exists
- Update status markers if behavior has changed
- Update examples or configuration snippets if they no longer match the code
- Remove references to removed concepts
