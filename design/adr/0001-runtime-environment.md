---
status: accepted
date: 2026-06-28
deciders: project maintainers
---

# Runtime Environment

## Status

accepted

## Context

SAKEM@S bot is a single-server Discord bot for a community of about 200 members. It is currently deployed on Shuttle. The bot is written in Rust and uses `poise`/`serenity`, `sqlx` with PostgreSQL, and `twapi-v2` for Twitter integration.

We need to choose a long-term runtime environment. The target environment must support:

- A continuously running Rust process (Discord Gateway connection)
- A relational database
- Low-latency outbound connections to Discord and voice servers in Japan
- Minimal operational cost

## Problem

The current deployment on Shuttle is a hosted platform that abstracts away servers and databases. While convenient, it ties the project to a single vendor and may limit future needs such as voice features (Songbird) or custom runtime behavior. We want to evaluate whether to remain on Shuttle or move to a different environment.

## Existing Design Assessment

The existing architecture is a single Rust binary with a Postgres pool and a `shuttle-runtime` entry point. There is no server management, container orchestration, or infrastructure-as-code. The runtime boundary is owned entirely by Shuttle.

Remaining on Shuttle requires no immediate code changes but leaves the runtime, networking, and pricing decisions outside the project’s control. It also does not prepare the project for features that need more control, such as persistent WebRTC voice connections.

## Alternatives

### Alternative A: Stay on Shuttle

Keep the current Shuttle deployment.

Benefits:
- No migration work
- Managed Postgres and secrets
- Simple `cargo shuttle deploy` workflow

Drawbacks:
- Vendor lock-in
- Pricing and feature changes are outside project control
- Songbird voice support may be awkward or impossible under Shuttle’s runtime model

### Alternative B: Fly.io

Move to Fly.io with a Tokyo-region machine and Fly Postgres or a volume-backed SQLite.

Benefits:
- Tokyo region available for low latency
- Native Rust/container support
- Managed Postgres option

Drawbacks:
- Free tier for continuously running machines is effectively gone as of 2025
- A 200-member bot would incur a small but ongoing monthly cost
- Pricing policy has changed recently and may change again

### Alternative C: Render or Railway

Use a hobby PaaS such as Render Web Services or Railway.

Benefits:
- Very low management burden
- Git-push deployment

Drawbacks:
- Render free tier puts services to sleep, which breaks a Discord bot
- Railway pricing is credit-based and not free for continuous workloads
- No Tokyo region, increasing latency to Discord Japan

### Alternative D: Hetzner Cloud VPS

Rent a small VPS in a European or US region.

Benefits:
- Excellent price/performance
- Full control over the OS and runtime

Drawbacks:
- No Japan region, so latency to Discord Japan voice servers is worse
- Requires manual OS/security management
- Not free

### Alternative E: Oracle Cloud Infrastructure Free Tier

Run the bot on an Always Free ARM VM in the Tokyo region, with a Docker-based Postgres or SQLite database.

Benefits:
- Always Free tier includes ARM VM capacity (up to 4 OCPU and 24 GB RAM across VMs)
- Tokyo and Osaka regions are available
- Full control over OS, containers, and runtime
- Sufficient resources for Songbird voice processing

Drawbacks:
- Higher setup and management burden than Shuttle
- Free-tier availability is best-effort and can be exhausted in popular regions
- Must be careful to select only Always Free shapes to avoid charges

### Alternative F: Self-Hosted Hardware

Run the bot on a Raspberry Pi or similar device at home, using Cloudflare Tunnel for external access.

Benefits:
- Effectively free after hardware purchase
- Complete control

Drawbacks:
- Reliability depends on home network and power
- Physical hardware maintenance
- Not suitable as the primary production runtime for a community bot

## Decision

Use **Oracle Cloud Infrastructure Free Tier** (Alternative E) as the new runtime environment. Deploy an Always Free ARM VM in the Tokyo region (`ap-tokyo-1`), run the bot in a Docker container, and use a containerized Postgres or SQLite database on the same VM.

Rationale:
- It is the only option that satisfies all constraints: free, Tokyo region, continuously running, enough RAM for Songbird, and full runtime control.
- The management burden is higher than Shuttle but lower than self-hosting because OCI still manages the underlying hardware and network fabric.
- It preserves the option to migrate to a paid VPS later without changing the application architecture.

## Consequences

Positive:
- Zero monthly cost under the Always Free tier
- Tokyo-region deployment minimizes latency to Discord Japan
- Container-based deployment makes future migrations easier
- Sufficient headroom to add Songbird voice features

Negative:
- Project must manage VM provisioning, security lists, and OS updates
- Free-tier capacity is not guaranteed; monitoring is required
- Secrets and database backups become the project’s responsibility
- Initial migration requires non-trivial effort

## Required Axes Impact

- **Encapsulation:** The runtime boundary moves from Shuttle into the project. Infrastructure details (VM, Docker, secrets) become internal concerns; they must not leak into command or event-handler logic.
- **Separation of concerns:** Runtime, database, and application logic remain distinct. The `main.rs` runtime entry point will change, but domain modules should remain untouched.
- **Invariants:** There is no runtime invariant in the application layer today. A new operational invariant will be “the bot process must restart automatically after VM reboot.”
- **Consistency:** `README.md` and any future `design/` documents must point to the single source of runtime truth in `design/adr/0001-runtime-environment.md`.
- **Fitness for purpose:** OCI Free Tier is more powerful and complex than a 200-member bot needs, but it is the only free option that satisfies the constraints. The extra headroom is acceptable because it enables future voice features without another migration.

## Required Design Updates

- Update `README.md` to reference `design/adr/` and note that the runtime environment is OCI Free Tier.
- Create follow-on ADRs for database choice, secrets management, containerization, and Songbird if voice features are pursued.
