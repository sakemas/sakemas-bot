# Northflank Sandbox Deployment Guide

> This guide documents how to deploy SAKEM@S bot to [Northflank](https://northflank.com) using the free Sandbox tier and the existing `Dockerfile`.
> It is a **feasibility study**: validate each step before treating it as production.
>
> Background: `design/adr/0007-northflank-sandbox-feasibility.md`

## Overview

Northflank is a developer platform that builds and runs containerized applications from a Git repository. With the free Sandbox tier you can create:

- 2 services (continuously running containers)
- 1 managed database addon
- 2 cron jobs

This guide uses:

- **Service 1**: the SAKEM@S bot, built from the repository `Dockerfile`
- **Database**: a managed PostgreSQL 16 addon
- **Region**: GCP `asia-northeast1` (Tokyo) for low latency to Discord Japan

## Prerequisites

- A [Northflank](https://app.northflank.com) account.
- This repository pushed to GitHub and linked to Northflank.
- Your local `.env` file with production secrets ready.
- A Discord bot token that is not currently running on another host (to avoid Gateway conflicts).

## Important notes

- **Do not commit `.env` or secrets to Git.** Northflank injects runtime variables through its dashboard.
- The `Dockerfile` already avoids copying `.env`; it relies on environment variables only.
- The bot has no HTTP server, so you do **not** need to expose any public port.
- The free Sandbox tier has undisclosed resource limits; monitor builds and runtime for OOM or timeout errors.

## Step 1: Create a Northflank project

1. Log in to [Northflank](https://app.northflank.com).
2. Click **Create project**.
3. Choose a name such as `sakemas-bot`.
4. Select the **Sandbox** plan (free).
5. For the default region, choose **GCP asia-northeast1** (Tokyo) if available. If not, choose the closest GCP region.

## Step 2: Link your GitHub repository

1. Go to **Settings → Git integrations** and connect your GitHub account.
2. Grant access to the `sakemas-bot` repository.
3. Return to the project dashboard.

## Step 3: Create the PostgreSQL addon

1. Click **Create new → Addon**.
2. Select **PostgreSQL**.
3. Name it `sakemas-bot-db`.
4. Version: **16** (matches the local `docker-compose.yml`).
5. Enable **TLS**.
6. Choose whether to make it **publicly accessible**. For a first test, enabling public access with TLS makes debugging easier. You can restrict it later.
7. Select the smallest resource plan to stay within the Sandbox tier.
8. Create the addon and wait for provisioning.
9. Open the addon and note the connection details:
   - `POSTGRES_URI` (connection string)
   - `HOST`, `PORT`, `USERNAME`, `PASSWORD`, `DATABASE`

## Step 4: Create the bot service

1. Click **Create new → Service**.
2. Choose **Combined service** (build + run in one).
3. Name it `sakemas-bot`.
4. Repository: select the linked GitHub repository and the branch (e.g., `oracle-cloud` or `main`).
5. Build options: choose **Dockerfile** and leave the path as `/` (root).
6. Networking: do **not** add any public ports. The bot does not expose HTTP.
7. Resources: start with the default Sandbox plan. Increase only if the build or runtime fails.
8. Click **Create service**.

The first build will start automatically. Songbird/audiopus dependencies can take several minutes to compile.

## Step 5: Configure runtime environment variables

1. Go to the `sakemas-bot` service dashboard.
2. Open **Environment** (or **Runtime variables**).
3. Add the following variables one by one:

```env
DISCORD_TOKEN=<your-bot-token>
TWITTER_CLIENT_ID=<your-twitter-client-id>
TWITTER_CLIENT_SECRET=<your-twitter-client-secret>
VC_ANNOUNCEMENT_CHANNEL=<channel-id>
WELCOME_CHANNEL=<channel-id>
CAUTION_CHANNEL=<channel-id>
INTRODUCTION_CHANNEL=<channel-id>
X_POSTER_CHANNEL=<channel-id>
```

4. Add `DATABASE_URL`. You can either:
   - copy the full `POSTGRES_URI` from the PostgreSQL addon connection details, or
   - link the addon to a secret group and inherit `POSTGRES_URI` as `DATABASE_URL` (recommended).

### Linking the database via a secret group (recommended)

1. In the project, go to **Secrets → Secret groups**.
2. Create a secret group named `sakemas-bot-secrets`.
3. Click **Show addons** and select the PostgreSQL addon.
4. Select `POSTGRES_URI` and alias it to `DATABASE_URL`.
5. Apply the secret group to the `sakemas-bot` service.
6. In the service environment page, confirm `DATABASE_URL` is inherited.

## Step 6: Deploy

1. Make sure the service build succeeded. If not, check the build logs.
2. The service should deploy automatically after the build.
3. Go to **Logs** and look for:
   - `migrations applied successfully` (or similar SQLx migration output)
   - `client.start()` running without errors
   - No repeated Gateway reconnections

## Step 7: Verify the bot

1. In Discord, check that the bot appears online in your server.
2. Run a slash command such as `/join` in a voice channel.
3. Test `/play-url` with a small HTTP audio stream or `/play-file` with an uploaded file.
4. Test a database-backed command such as `/list_vc_announcements`.

## Step 8: Monitor and tune

1. Watch **Metrics** in the Northflank dashboard for CPU and memory usage.
2. If the build times out or runs out of memory:
   - enable build layer caching in the build settings,
   - or split the build into a separate build service with more resources (may require upgrading from Sandbox).
3. If the runtime service OOMs:
   - increase the memory allocation if the Sandbox plan allows it,
   - or consider falling back to a VPS.

## Updating the bot

1. Push a new commit to the linked branch.
2. Northflank will build and deploy automatically if CI/CD is enabled.
3. Monitor logs for migration errors or startup failures.

## Fallback

If Northflank Sandbox cannot host the bot reliably:

- Use `docs/deployment.md` (OCI Docker Compose) once ARM capacity is available.
- Use `docs/deployment-nixos.md` if you decide to deploy NixOS on available hardware.
- Consider a budget Tokyo VPS such as Contabo Cloud VPS and run Docker Compose there.

## Cost

Northflank Sandbox is free for qualifying workloads. Monitor usage in the dashboard. If usage exceeds the Sandbox limits, you will need to upgrade to a paid plan or switch provider.
