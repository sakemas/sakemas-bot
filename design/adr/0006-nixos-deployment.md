---
status: proposed
date: 2026-06-29
deciders: project maintainers
---

# NixOS Deployment Feasibility

## Status

proposed

## Context

`design/adr/0004-containerization.md` selected Docker and Docker Compose for running SAKEM@S bot and PostgreSQL on a single OCI Free Tier ARM VM in Tokyo. Docker Compose is already implemented: `Dockerfile`, `docker-compose.yml`, and `docs/deployment.md` exist and build successfully.

The maintainer asked whether NixOS could replace the Docker Compose stack, motivated by:

- a fully declarative system configuration (OS, database, bot service, firewall),
- reduced runtime moving parts (no Docker daemon, no Compose runtime),
- the possibility of building the entire boot disk as a versioned artifact,
- keeping the deployment free of charge and simple to operate.

## Problem

We need to decide whether NixOS provides a net benefit over the accepted Docker Compose path for this single-server, hobby project. The evaluation must be grounded in an actual prototype, not only in NixOS advocacy.

## Existing Design Assessment

Docker Compose already satisfies the runtime requirements:

- `docker-compose.yml` is the single source of runtime topology.
- PostgreSQL persists in a Docker volume.
- The bot container depends on the database container and restarts automatically.
- The setup is documented in `docs/deployment.md`.

A NixOS alternative would not extend the existing Docker abstraction. It would replace it with a NixOS system configuration that manages PostgreSQL as a native NixOS service and the bot as a systemd unit. This is a different boundary: the entire VM is the artifact, not a set of containers.

## Alternatives

### Alternative A: Keep Docker Compose (status quo)

Continue with the accepted Docker Compose setup.

Benefits:

- Already implemented and validated on ARM64.
- Familiar to the maintainer.
- Easy to reproduce locally with `docker compose up`.
- PostgreSQL upgrades and backups follow well-documented container patterns.

Drawbacks:

- Docker daemon is an extra service on the VM.
- Reproducibility depends on image tags and manual VM configuration.
- The VM OS itself is not declarative.

### Alternative B: NixOS system configuration with `oci-image.nix`

Build a complete NixOS qcow2 image for OCI. The image includes the bot package, a PostgreSQL NixOS service, a systemd unit for the bot, and SSH access. The image is built on an `aarch64-linux` host or remote builder.

Benefits:

- The entire system (kernel, bootloader, PostgreSQL, bot, firewall) is declared in the flake.
- No Docker daemon or Compose runtime on the deployed VM.
- Rollbacks are possible via NixOS generations.
- Secrets stay out of the Nix store if loaded via `EnvironmentFile`.

Drawbacks:

- Requires an `aarch64-linux` builder (e.g., an OCI ARM VM) to produce the image.
- The maintainer must learn NixOS concepts (modules, generations, store).
- Secrets file must be placed on the running VM after the image boots.
- PostgreSQL credentials need a manual bootstrap step unless a secrets manager (e.g., sops-nix, agenix) is added, which increases complexity.
- OCI image upload and custom-image import add steps outside the repository.

### Alternative C: NixOS host running Docker Compose via `arion` or `virtualisation.oci-containers`

Use NixOS only to provision the VM and run the existing Docker Compose stack declaratively (for example, with `arion` or `virtualisation.oci-containers`).

Benefits:

- Keeps the existing `docker-compose.yml` mostly intact.
- Gains declarative OS provisioning without rewriting the application runtime.

Drawbacks:

- Adds NixOS complexity without removing Docker.
- Two abstraction layers (NixOS + containers) instead of one.
- Does not reduce runtime moving parts.

## Decision

**Proposed:** Adopt Alternative B (NixOS system configuration with `oci-image.nix`) **if and only if** the following validation steps succeed on actual OCI hardware:

1. `nix build .#packages.aarch64-linux.oci-image` completes on an `aarch64-linux` builder.
2. The produced qcow2 image boots as an OCI custom image.
3. PostgreSQL initializes and the `sakemas_bot` database and user exist.
4. After placing `/etc/sakemas-bot/secrets.env` with correct systemd `EnvironmentFile` format, the bot service starts and connects to Discord.
5. PostgreSQL credentials are bootstrapped (manual password set) and match the value in `secrets.env`.
6. A reboot leaves the bot running without manual intervention.

Until those steps are completed, **Alternative A remains the accepted path**. In addition, `design/adr/0007-northflank-sandbox-feasibility.md` proposes using Northflank Sandbox as an interim runtime while OCI capacity is unavailable. If validation succeeds, this ADR will be updated to `accepted` and will supersede `design/adr/0004-containerization.md`.

Rationale for the proposed direction:

- The project values simplicity and low cost, but it also values reproducibility and single-source-of-truth configuration. A working NixOS flake would make the whole VM reproducible from one repository.
- The prototype already exists: `flake.nix` defines `nixosConfigurations.sakemas-oci`, and `nix flake check --no-build` passes.
- The main blocker is hardware access for the final image build and boot test, not a design flaw in the NixOS approach.

## Consequences

Positive if accepted:

- One repository defines the application code, package, database, and OS.
- VM configuration drift is eliminated.
- NixOS generations provide a recovery path after bad changes.
- No Docker daemon to update or debug.

Negative if accepted:

- The maintainer must operate a NixOS system, which has a learning curve.
- Image builds require an `aarch64-linux` environment (initial OCI VM, remote builder, or QEMU emulation).
- Secrets bootstrap is a manual post-boot step unless a secrets manager is added.
- PostgreSQL upgrades and major NixOS channel updates require NixOS-specific knowledge and may need data migration.
- If validation fails, effort spent on NixOS prototyping is partially lost.

## Required Axes Impact

- **Encapsulation:** The runtime moves from `docker-compose.yml` to `nix/nixos-modules/sakemas-bot.nix` and `flake.nix`. Application code still reads environment variables and remains unaware of NixOS.
- **Separation of concerns:** PostgreSQL, the bot, SSH, and firewall are separate NixOS modules with explicit dependencies. The bot service declares `after`/`requires` on `postgresql.service`.
- **Invariants:** The invariant "bot connects to the configured PostgreSQL URL" is enforced by the NixOS module loading the same `DATABASE_URL` from `EnvironmentFile`. The invariant "secrets do not enter the Nix store" is enforced by keeping `secrets.env` outside the flake.
- **Consistency:** A single flake becomes the source of truth for packages, development shells, and the OCI image. This is more consistent than Docker Compose + manually configured VM OS.
- **Fitness for purpose:** NixOS matches the goal of a small, reproducible, single-server deployment. Kubernetes or managed container services would be broader than needed.

## Required Design Updates

If this ADR is accepted:

- Update `flake.nix` and `nix/nixos-modules/sakemas-bot.nix` based on boot-test feedback.
- Create `docs/deployment-nixos.md` with OCI image build, upload, and bootstrap steps.
- Mark `design/adr/0004-containerization.md` as superseded by this ADR.
- Update `README.md` to reference the NixOS deployment path.
- Define a secrets bootstrap and backup procedure.

If this ADR is rejected:

- Remove or archive the NixOS prototype files.
- Keep `design/adr/0004-containerization.md` as accepted.
