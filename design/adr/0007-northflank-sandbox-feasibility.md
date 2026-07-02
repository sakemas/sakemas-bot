---
status: proposed
date: 2026-07-01
deciders: project maintainers
---

# Northflank Sandbox Feasibility

## Status

proposed

## Context

`design/adr/0001-runtime-environment.md` selected Oracle Cloud Infrastructure (OCI) Free Tier as the production runtime, and `design/adr/0004-containerization.md` adopted Docker Compose on a single VM. `design/adr/0006-nixos-deployment.md` later proposed NixOS as a possible replacement for the Docker Compose stack.

In practice, OCI Free Tier ARM capacity in the Tokyo region (`ap-tokyo-1`) has been unavailable for this account, and NixOS image builds require an `aarch64-linux` host that we do not currently have. We need a practical way to get the bot running in production quickly without waiting for hardware availability.

Northflank offers a free Sandbox tier that includes always-on compute, two services, one managed database, and two cron jobs. It supports Dockerfile-based builds from GitHub and managed PostgreSQL addons. The platform also supports deploying into Google Cloud Platform regions, including Tokyo (`asia-northeast1`).

## Problem

We need to determine whether Northflank Sandbox can host SAKEM@S bot with PostgreSQL at no cost, with acceptable reliability and latency for a 200-member Discord server. The evaluation must be based on an actual deployment, not only on documentation.

Key open questions:

- Do the free Sandbox resource limits allow the bot binary, PostgreSQL, and Songbird voice processing to run together?
- Does Northflank’s always-on free tier keep the Discord Gateway connection alive continuously?
- Can the existing `Dockerfile` build within Northflank’s build environment and time limits?
- Is latency to Discord Japan acceptable from a GCP Tokyo deployment?
- How complex is secret and database connection management compared to Docker Compose on a VPS?

## Existing Design Assessment

The accepted Docker Compose path (`docker-compose.yml` + `Dockerfile`) already packages the bot and PostgreSQL together. Northflank would not extend that abstraction directly. Instead, it splits the stack into:

- a managed PostgreSQL addon,
- a continuously running service built from the same `Dockerfile`,
- runtime environment variables injected through the Northflank dashboard.

The application code does not need to change: it still reads `DATABASE_URL`, `DISCORD_TOKEN`, and other values from the process environment. The `dotenvy` call in `src/main.rs` is harmless when no `.env` file is present, because Northflank injects variables directly.

## Alternatives

### Alternative A: Wait for OCI Free Tier ARM capacity

Continue retrying OCI Free Tier ARM VM creation in Tokyo until capacity becomes available.

Benefits:

- Fully aligned with the accepted runtime decision.
- No new ADR or documentation needed.
- NixOS path remains viable once hardware is available.

Drawbacks:

- No predictable timeline for capacity availability.
- The bot remains without a production deployment.
- Waiting does not reduce risk; it only delays validation.

### Alternative B: Contabo Tokyo VPS (or similar budget VPS)

Provision a paid VPS in Tokyo (e.g., Contabo Cloud VPS at €8.05/month) and deploy Docker Compose or NixOS there.

Benefits:

- Guaranteed capacity.
- Enough resources for bot + PostgreSQL + Songbird.
- Keeps the Tokyo-region requirement.

Drawbacks:

- Not free, conflicting with the strongest cost constraint.
- Adds ongoing monthly cost and vendor relationship.
- Still requires manual VM setup if using Docker Compose.

### Alternative C: Northflank Sandbox

Use Northflank’s free Sandbox tier to deploy the bot service and a managed PostgreSQL addon in the GCP Tokyo region.

Benefits:

- Potentially zero cost for the target workload.
- Managed PostgreSQL reduces operational burden.
- Git-push deployment via Dockerfile.
- Tokyo region available through GCP.
- No VM OS management.

Drawbacks:

- Free-tier resource limits are not fully documented; the bot may not fit.
- Build times for Rust + Songbird dependencies may exceed Sandbox limits.
- NixOS is not supported, so the NixOS prototype cannot be reused.
- Managed PostgreSQL is a separate service with its own networking and credentials.
- Vendor dependency on Northflank, a smaller platform than OCI or GCP.

## Decision

**Proposed:** Adopt Alternative C (Northflank Sandbox) as a short-term, experimental production runtime while Alternative A remains the long-term target.

Rationale:

- It directly addresses the current blocker (no OCI capacity) without waiting indefinitely.
- It keeps the deployment free or very low cost, satisfying the strongest project constraint.
- It uses the existing `Dockerfile`, so application code changes are minimal.
- It provides a managed database, reducing one operational task.
- If validation fails, the project can fall back to Alternative B with only documentation and environment-variable changes.

## Validation Steps

This ADR remains `proposed` until the following steps are completed:

1. Northflank account created and GitHub repository linked.
2. PostgreSQL addon provisioned in the GCP Tokyo region.
3. Bot service created using the repository’s `Dockerfile`.
4. Runtime environment variables and database connection secrets configured.
5. Service builds successfully and deploys.
6. Bot connects to Discord Gateway and stays online for at least 24 hours.
7. Slash commands respond correctly.
8. Songbird voice commands (join, leave, play-url, stop) work in a VC.
9. PostgreSQL migrations run and data persists across restarts.
10. Monthly resource usage stays within the free Sandbox limits.

If all steps pass, this ADR will be marked `accepted` and will become the active runtime decision. The accepted Docker Compose path will be demoted to a fallback, and the NixOS path in ADR 0006 will remain `proposed` for future evaluation.

If any validation step fails in a way that cannot be resolved within the free tier, this ADR will be marked `rejected` and the project will evaluate Alternative B.

## Consequences

Positive if accepted:

- Bot is running in production quickly and at no cost.
- Managed PostgreSQL reduces backup and upgrade responsibility.
- Deployment is driven by Git commits, simplifying updates.
- Tokyo region keeps latency to Discord Japan low.

Negative if accepted:

- NixOS prototype is not used; declarative system configuration is lost.
- Dependency on Northflank’s free-tier policies, which may change.
- Resource limits may constrain future features.
- Secrets management moves to a third-party UI; recovery depends on Northflank access.

## Required Axes Impact

- **Encapsulation:** The runtime boundary moves from a self-managed VM to Northflank’s managed services. Application code remains encapsulated; it still reads environment variables.
- **Separation of concerns:** Database and bot are separate Northflank services with distinct lifecycles. Connection details are injected via secret groups.
- **Invariants:** The invariant “bot connects to the configured PostgreSQL URL” is enforced by injecting `DATABASE_URL` as a runtime variable. The invariant “secrets do not enter the image” is enforced by not copying `.env` into the Docker image.
- **Consistency:** `Dockerfile` remains the build source of truth. `docker-compose.yml` becomes a local-development-only artifact.
- **Fitness for purpose:** Northflank Sandbox is broader than needed if it succeeds, but it is narrower than Kubernetes or a multi-VM setup. It matches the hobby scale of the project.

## Required Design Updates

If this ADR is accepted:

- Create `docs/deployment-northflank.md` with step-by-step instructions.
- Update `README.md` to note Northflank as the current runtime and Docker Compose as the local/fallback path.
- Keep `design/adr/0004-containerization.md` and `docs/deployment.md` as fallback documentation.
- Keep `design/adr/0006-nixos-deployment.md` as `proposed` for future hardware availability.

If this ADR is rejected:

- Remove `docs/deployment-northflank.md` or mark it as experimental.
- Update `README.md` to reflect the fallback runtime decision.
- Proceed with Alternative B (budget VPS) and update relevant ADRs.
