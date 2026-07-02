---
status: accepted
date: 2026-06-28
deciders: project maintainers
---

# Containerization

## Status

accepted

## Context

After deciding to run SAKEM@S bot on an OCI Free Tier ARM VM in Tokyo (`design/adr/0001-runtime-environment.md`) with a PostgreSQL database on the same VM (`design/adr/0002-database-selection.md`), we must choose how the bot binary and the database are packaged, networked, and kept running.

The deployment has only two moving parts: the Rust bot and PostgreSQL. The load is low, but the bot must stay online continuously and recover from VM reboots.

## Problem

We need a deployment strategy that:

- runs on the selected OCI Free Tier ARM VM,
- hosts both the bot and PostgreSQL,
- keeps them connected and persistent,
- restarts automatically after failures or reboots,
- is simple enough to maintain by the project maintainers.

## Existing Design Assessment

There is no existing containerization or deployment design beyond Shuttle’s managed runtime. The project does not have a `Dockerfile`, `docker-compose.yml`, or systemd unit. This means we are free to choose the simplest viable option rather than extend an existing abstraction.

## Alternatives

### Alternative A: Docker and Docker Compose on the VM

Build a container image for the bot and orchestrate it with PostgreSQL via `docker-compose.yml`.

Benefits:
- Single file describes the entire runtime (`docker-compose.yml`)
- PostgreSQL has an official, well-tested ARM image
- Networking between containers is handled by Docker Compose
- `restart: always` gives automatic recovery
- Familiar tooling; easy to reproduce locally
- Future migration to another host requires only copying the compose file, env file, and volumes

Drawbacks:
- Docker daemon is an additional service to keep running
- Slightly more overhead than a bare binary
- Volume backups must be configured manually

### Alternative B: Bare-Metal Binary with Systemd

Compile the Rust binary directly on the VM (or cross-compile), run it as a systemd service, and run PostgreSQL either natively or in Docker.

Benefits:
- No container image build step
- Lower runtime overhead
- Direct control over the binary and its environment

Drawbacks:
- Native PostgreSQL installation and upgrades are more work than a container
- Development/production parity is weaker (developer likely uses Docker or Shuttle Postgres locally)
- Systemd unit files, log rotation, and dependency startup must be managed
- Cross-compilation for ARM can add friction to the build process

### Alternative C: Podman with Quadlet/Systemd

Use Podman (rootless or rootful) with Quadlet files or systemd units to run containers.

Benefits:
- Daemonless container runtime
- Rootless containers possible, improving security
- Compatible with Docker Compose files via `podman-compose`

Drawbacks:
- Less familiar than Docker in this project context
- Rootless networking and volume permissions can be subtle
- ARM image availability and compatibility must be verified
- Adds learning curve without clear benefit for a single-server hobby project

## Decision

Use **Docker and Docker Compose** (Alternative A).

Rationale:
- The project has only two services, making Docker Compose the natural fit.
- Official PostgreSQL and Rust base images support ARM64, so the OCI ARM VM is fully compatible.
- `docker-compose.yml` becomes a single source of truth for the runtime topology.
- The same compose file can be used for local development, reducing environment drift.
- Restart policy and volume persistence are built-in.
- It is the lowest-friction path for a maintainer who already knows Docker.

## Consequences

Positive:
- Reproducible runtime on VM and locally
- Clear separation between bot and database lifecycles
- Easy to add a third container later (e.g., a backup sidecar)
- ARM64 base images are widely available

Negative:
- Docker daemon must be running and occasionally updated
- Volume backups are the maintainer’s responsibility
- Image builds may be slow on a 1–2 OCPU ARM VM

## Required Axes Impact

- **Encapsulation:** The runtime topology is encapsulated in `docker-compose.yml`. Application code remains unaware of whether it runs in a container.
- **Separation of concerns:** Bot container and database container are separate. The bot depends on the database via a network, not by sharing files or state.
- **Invariants:** The invariant “bot connects to the configured PostgreSQL URL” is enforced by compose environment variables and startup order. The database data must persist across container restarts, enforced by a named volume.
- **Consistency:** The compose file is the single source of runtime topology. Documentation such as `README.md` should reference it rather than describing manual steps.
- **Fitness for purpose:** Docker Compose matches the two-service scale of this project. Kubernetes, bare-metal systemd, or Podman would be broader than needed.

## Required Design Updates

- Add `Dockerfile` and `docker-compose.yml` to the migration implementation plan.
- Update `README.md` deployment instructions to reference Docker Compose.
- Define a volume backup strategy in the migration plan or a follow-on operational document.
