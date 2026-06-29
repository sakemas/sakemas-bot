# Architecture Decision Records

This directory contains the architectural decisions that shape SAKEM@S bot. Each ADR is a durable record of why a particular choice was made, not just what was chosen.

## Status Labels

- `draft` — being written, not yet ready for review
- `proposed` — ready for review, decision not yet made
- `accepted` — decision adopted
- `rejected` — decision considered and rejected
- `superseded` — replaced by a newer ADR

## Adding a New ADR

1. Copy `template.md` to `NNNN-title.md` using the next available number.
2. Fill in all sections, especially the problem and alternatives.
3. Run the `adr-review` skill before marking it `accepted`.
4. Update this index.

## Accepted ADRs

| Number                              | Title                | Status   |
| ----------------------------------- | -------------------- | -------- |
| [0001](0001-runtime-environment.md) | Runtime Environment  | accepted |
| [0002](0002-database-selection.md)  | Database Selection   | accepted |
| [0003](0003-secrets-management.md)  | Secrets Management   | accepted |
| [0004](0004-containerization.md)    | Containerization     | accepted |
| [0005](0005-songbird.md)            | Songbird Integration | accepted |

## Proposed ADRs

| Number                           | Title                        | Status   |
| -------------------------------- | ---------------------------- | -------- |
| [0006](0006-nixos-deployment.md) | NixOS Deployment Feasibility | proposed |
