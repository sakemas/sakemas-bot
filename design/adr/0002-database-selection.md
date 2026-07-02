---
status: accepted
date: 2026-06-28
deciders: project maintainers
---

# Database Selection

## Status

accepted

## Context

SAKEM@S bot currently stores its state in a PostgreSQL database managed through Shuttle. The runtime environment decision (`design/adr/0001-runtime-environment.md`) selects Oracle Cloud Infrastructure Free Tier. We must now decide which database engine to use on OCI.

The bot serves a single Discord server of about 200 members. Concurrent writes are low. The current schema is small: tables for VC announcements, idols, calendar events, and Twitter OAuth tokens.

## Problem

Moving to OCI removes the managed Shuttle Postgres. We need a database strategy that:

- works on the selected OCI Free Tier ARM VM,
- preserves existing schema and queries where possible,
- keeps operational burden low,
- remains free.

## Existing Design Assessment

The codebase is tightly coupled to PostgreSQL:

- `Cargo.toml` depends on `shuttle-shared-db` with the `postgres` and `sqlx` features.
- `src/main.rs` receives a `sqlx::PgPool` from the Shuttle runtime.
- `src/utils/twitter/access_token.rs`, `src/commands/idol.rs`, and `src/commands/vc_announcement.rs` use `sqlx::query` and `sqlx::query_as` with PostgreSQL `$1`, `$2` positional parameters.
- All migration files under `migrations/` use PostgreSQL-specific syntax:
  - `SERIAL` for auto-incrementing primary keys
  - `VARCHAR` without length
  - `TIMESTAMP WITH TIME ZONE`
  - `CHAR_LENGTH`
  - `INT8`
  - `ON DELETE CASCADE` foreign keys

These details are spread across the runtime entry point, command modules, utility modules, and migration files. Changing the database engine requires touching every SQL boundary in the project.

## Alternatives

### Alternative A: PostgreSQL in Docker on OCI

Run the official PostgreSQL container on the same ARM VM as the bot.

Benefits:
- No migration or query rewrites needed
- Existing `sqlx::PgPool` and `sqlx::migrate!()` continue to work
- Familiar tooling and backup workflows
- Handles the low write concurrency of this bot easily

Drawbacks:
- Uses additional RAM on the VM (a few hundred MB)
- Requires container management and PostgreSQL version selection
- Backups become the project’s responsibility

### Alternative B: SQLite on OCI Volume

Replace PostgreSQL with SQLite stored on an OCI block volume.

Benefits:
- Lower memory footprint than PostgreSQL
- Simpler single-file deployment
- No separate container to manage

Drawbacks:
- All migrations must be rewritten with SQLite syntax (`INTEGER PRIMARY KEY AUTOINCREMENT` instead of `SERIAL`, `TEXT` instead of `VARCHAR`, `LENGTH` instead of `CHAR_LENGTH`, `INTEGER` instead of `INT8`)
- All SQL queries must switch from `$1` placeholders to SQLite `?1` placeholders
- `sqlx` setup changes from `PgPool` to `SqlitePool`
- SQLite has limited `ALTER TABLE` support, making future schema changes more awkward
- Single-writer concurrency requires careful handling if multiple tasks write at once

Given the current PostgreSQL coupling, the migration cost is high.

### Alternative C: Oracle Autonomous Database

Use the OCI Always Free Autonomous Database.

Benefits:
- Fully managed by Oracle
- No VM-side database administration

Drawbacks:
- It is Oracle Database, not PostgreSQL
- `sqlx` support for Oracle is not as mature as for Postgres
- Existing migrations and queries would require significant rewriting
- Adds a separate network dependency and connection management

The migration effort is even larger than SQLite and introduces a less familiar toolchain.

## Decision

Use **PostgreSQL in Docker on the same OCI ARM VM** (Alternative A).

Rationale:
- It is the only alternative that avoids rewriting the existing SQL layer.
- The OCI Free Tier ARM VM has enough RAM to run both the bot and a small PostgreSQL container.
- A 200-member server produces negligible database load, so a co-located container is sufficient.
- Operational patterns (pg_dump backups, migrations, psql debugging) stay the same.

## Consequences

Positive:
- Near-zero application code changes for the database layer
- Existing migrations remain valid
- Existing `sqlx` compile-time checks continue to work
- Future schema changes follow the current migration workflow

Negative:
- VM must reserve memory and CPU for PostgreSQL
- Database backups must be configured manually
- A single VM hosts both application and database, so VM failure affects both

## Required Axes Impact

- **Encapsulation:** The database engine remains behind the `PgPool` abstraction. The migration only changes how the pool is constructed (local Docker connection string instead of Shuttle-injected pool).
- **Separation of concerns:** Domain modules continue to own queries; infrastructure owns connection setup and migrations. No query logic moves between modules.
- **Invariants:** Schema invariants remain in `migrations/`. The migration owner does not change.
- **Consistency:** `migrations/` stays the single source of schema truth. No duplicate schema definitions are introduced.
- **Fitness for purpose:** PostgreSQL is heavier than a 200-member bot needs, but it is the correct fit because the project already depends on it. Migrating to a lighter engine would be less fit for the immediate goal of a low-risk move.

## Required Design Updates

- Update `README.md` database references if any mention Shuttle Postgres explicitly.
- Create a follow-on ADR or implementation plan for container orchestration and database backups on OCI.
