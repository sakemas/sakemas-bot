---
name: document-date-accuracy
description: Anchor document dates and relative-time claims to the OS clock and mark unstable current-state claims explicitly.
---

# Document Date Accuracy

## Purpose

Prevent stale or model-inferred dates in documents. Any document that mentions time must anchor that claim to a verifiable source.

## Rules

When creating or editing documents that include dates, relative time, timelines, release dates, deadlines, schedules, milestones, or words such as `today`, `recent`, `latest`, `current`, `now`, `as of`, `last`, or `next`:

1. Get the anchor timestamp from the OS:
   ```sh
   date -u +"%Y-%m-%d %H:%M:%S UTC %z"
   ```

2. Convert relative dates to exact calendar dates where possible.

3. Use UTC for document time notation.

4. Mark unstable or planned current-state claims explicitly, for example with `[as of YYYY-MM-DD]` or `[planned]`.

5. Do not rely on model memory for the current date.

## Examples

Avoid:

> The free tier currently offers 24 GB of RAM.

Prefer:

> As of 2026-06-29, the free tier offers 24 GB of RAM.

Avoid:

> We will migrate next month.

Prefer:

> Target migration date: 2026-07-31.
