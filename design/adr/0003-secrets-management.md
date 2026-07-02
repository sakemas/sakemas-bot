---
status: accepted
date: 2026-06-28
deciders: project maintainers
---

# Secrets Management

## Status

accepted

## Context

SAKEM@S bot currently runs on Shuttle and receives secrets through the `shuttle_runtime::SecretStore` abstraction. After the runtime move to Oracle Cloud Infrastructure Free Tier (`design/adr/0001-runtime-environment.md`), this abstraction disappears. We must choose a new way to supply secrets to the bot.

Secrets currently in use:

- `DISCORD_TOKEN` — read once at startup in `src/main.rs`
- `TWITTER_CLIENT_ID` — read at startup via `set_env_var` and later at runtime in `src/utils/twitter/access_token.rs`
- `TWITTER_CLIENT_SECRET` — same as above
- `VC_ANNOUNCEMENT_CHANNEL`, `WELCOME_CHANNEL`, `CAUTION_CHANNEL`, `INTRODUCTION_CHANNEL`, `X_POSTER_CHANNEL` — read at startup via `set_env_var` and then from `std::env::var` in `src/utils/channel.rs`

There is no dynamic secret rotation today; all values are static configuration.

## Problem

We need a secret-loading mechanism that:

- works outside Shuttle,
- is simple enough for a single-server hobby project,
- keeps secrets out of the source tree,
- supports both startup-time and runtime reads with minimal code changes.

Additionally, `Secrets.toml` and `Secrets.dev.toml` currently contain real Discord and Twitter tokens and are committed to the repository. This is an existing security risk that should be addressed during the migration.

## Existing Design Assessment

Secrets are accessed in two ways:

1. **Shuttle `SecretStore`** (`src/utils/secret.rs`) — a runtime-provided map of key/value strings. `get_secret` reads a value; `set_env_var` reads a value and writes it to `std::env`.
2. **Environment variables** — `src/utils/channel.rs` reads channel IDs from `std::env::var` at call time. `src/utils/twitter/access_token.rs` reads Twitter credentials from `SecretStore` at call time.

The current abstraction is thin. Replacing `SecretStore` with another key/value source is straightforward, but two patterns need special attention:

- Channel IDs are passed through `std::env` because the `Channel` enum has no access to `Data`.
- Twitter credentials are read from `SecretStore` at runtime through a reference in `Data`.

## Alternatives

### Alternative A: Dotenv File on the VM

Store secrets in a `.env` file on the OCI VM and load them with `dotenvy` at process startup. `Secrets.toml` is removed from the repo.

Benefits:
- One change at startup replaces the entire Shuttle `SecretStore`
- Existing `std::env::var` reads continue to work without modification
- Runtime reads of Twitter credentials can be replaced by env reads or by passing values into `Data`
- Simple to understand and debug
- No extra network services

Drawbacks:
- Secrets live in a plain file on disk; VM compromise exposes them
- File permissions must be restricted (e.g., `chmod 600`)
- No audit or rotation features
- The `.env` file must be copied to the VM out-of-band

### Alternative B: Systemd Credentials

Use systemd’s `LoadCredential=` mechanism to pass secrets as files to a systemd service.

Benefits:
- Secrets are stored as files with explicit ownership and permissions
- No secrets in the process environment (depending on configuration)
- Integrates well with an autostart service

Drawbacks:
- Adds systemd-specific configuration
- Still requires files to be placed on the VM securely
- Code must be updated to read files instead of env vars for runtime reads
- Overkill for a single-container hobby deployment

### Alternative C: Docker Secrets

Use Docker secrets (bind-mounted files in compose) to pass secrets into the container.

Benefits:
- Secrets are files inside the container, not environment variables
- Works naturally with the planned Docker deployment

Drawbacks:
- Docker secrets in non-swarm mode are just bind mounts; the security gain is modest
- Requires changing `set_env_var` and runtime reads to read files
- More moving parts than a `.env` file

### Alternative D: OCI Vault

Store secrets in Oracle Cloud Infrastructure Vault and fetch them at startup via the OCI SDK.

Benefits:
- Managed, auditable secret storage
- Centralized rotation support
- Cloud-native integration

Drawbacks:
- Requires IAM configuration and SDK dependency
- Adds network dependency at startup
- Free tier key management has limits
- Significant over-engineering for a single-server hobby bot

## Decision

Use a **dotenv file loaded at startup** (Alternative A).

Rationale:
- The current code already relies on environment variables for channel IDs, and a dotenv loader is the smallest change that replaces Shuttle `SecretStore`.
- For a single-server hobby project, the operational overhead of OCI Vault or systemd credentials outweighs the security benefit.
- The `.env` file can be restricted to the deployment user on the VM (`chmod 600`).
- Future migration to OCI Vault or Docker secrets is possible without touching domain logic because the change is isolated to secret-loading code.

Security remediation:
- Remove `Secrets.toml` and `Secrets.dev.toml` from version control.
- Add them to `.gitignore`.
- Rotate the Discord and Twitter tokens that were exposed in git history, since they have been public in the repository.
- Provide a `.env.example` file with dummy values for local development.

## Consequences

Positive:
- Minimal code changes at startup
- Existing `std::env::var` reads keep working
- Easy to replicate in development and production
- No cloud-provider-specific secret service to manage

Negative:
- Secrets are stored in a plain file on the VM
- File permissions and VM security become the project’s responsibility
- No automatic rotation or audit trail

## Required Axes Impact

- **Encapsulation:** Secret loading remains isolated in `src/utils/secret.rs`. The rest of the application reads env vars or values injected into `Data`.
- **Separation of concerns:** Domain modules do not change. Only the startup and utility modules that access `SecretStore` are affected.
- **Invariants:** No runtime invariant changes. The new invariant is: secrets must not be present in the Git repository.
- **Consistency:** `.env.example` becomes the single source of truth for required secret names; production `.env` is derived from it.
- **Fitness for purpose:** A dotenv file is the right abstraction level for a hobby project. OCI Vault or systemd credentials would be broader than the concrete need.

## Required Design Updates

- Update `.gitignore` to exclude `Secrets.toml`, `Secrets.dev.toml`, and `.env`.
- Create `.env.example` with placeholder values.
- Update `README.md` development instructions to mention `.env` instead of `Secrets.dev.toml`.
- Add the secret-loading implementation to the migration plan.
