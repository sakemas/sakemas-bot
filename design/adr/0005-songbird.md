---
status: accepted
date: 2026-06-28
deciders: project maintainers
---

# Songbird Integration

## Status

accepted

## Context

The maintainers want to add Discord voice capabilities to SAKEM@S bot so that users can play music through commands. The runtime environment is an Oracle Cloud Infrastructure Free Tier ARM VM in Tokyo (`design/adr/0001-runtime-environment.md`), and the bot runs in Docker (`design/adr/0004-containerization.md`).

Songbird is the Rust voice library commonly used with Serenity. Adding it affects dependencies, container size, CPU use, and legal exposure depending on the audio source.

## Problem

We need to decide:

- whether to add Songbird now or later,
- which audio sources are acceptable,
- what the initial command surface should be,
- how to handle the runtime requirements (Opus/FFmpeg, CPU, network).

## Existing Design Assessment

The bot currently has no voice features. Adding Songbird requires:

- A new dependency in `Cargo.toml` (`serenity` with voice features, `songbird`).
- A new set of slash commands for voice channel management and playback.
- An audio source abstraction that fits the selected source type.
- Updates to the Docker image to include FFmpeg and possibly yt-dlp.

None of the existing modules own voice logic, so a new `commands/voice.rs` or similar module is appropriate.

## Alternatives

### Alternative A: Add Songbird with Local/HTTP-Stream Sources Only

Integrate Songbird and support playback from local files or direct HTTP audio streams.

Benefits:
- Directly satisfies the user requirement
- Local files avoid network dependency during playback
- HTTP streams can be radio stations or self-hosted content
- Legal risk is low when sources are owned or public-domain

Drawbacks:
- Adds FFmpeg and Opus encoding to the container
- Increases CPU and memory use on the ARM VM
- Requires users to upload files or provide stream URLs
- Some Discord regions may have stricter voice gateway behavior

### Alternative B: Add Songbird with YouTube/External Platform Support

Integrate Songbird and use yt-dlp or similar tools to play audio from YouTube and other platforms.

Benefits:
- Users can play almost any song by URL

Drawbacks:
- YouTube Terms of Service prohibit scraping and automated downloading in most contexts
- yt-dlp breakage is common and requires ongoing maintenance
- Audio extraction is CPU-intensive on ARM
- Legal and ToS risk for the project and the server

This alternative is rejected.

### Alternative C: Do Not Add Songbird

Keep the bot text-only and defer voice features indefinitely.

Benefits:
- Simpler runtime
- Smaller container
- No legal exposure

Drawbacks:
- Does not satisfy the stated user requirement

This alternative is rejected because the requirement is explicit.

### Alternative D: Use a Separate Music Bot

Rely on an existing open-source music bot instead of adding the feature to SAKEM@S bot.

Benefits:
- No development or maintenance effort
- Feature-rich playback

Drawbacks:
- Requires inviting another bot to the server
- Feature and policy changes are outside project control
- Does not integrate with SAKEM@S-specific commands or roles

This alternative is rejected because the goal is to add the capability to SAKEM@S bot itself.

## Decision

Add **Songbird with local files and direct HTTP streams only** (Alternative A).

Rationale:
- It satisfies the user requirement while avoiding the legal and maintenance risks of YouTube scraping.
- Local files are reliable for a small server and do not depend on external platforms.
- HTTP streams can support radio or self-hosted content without scraping.
- The OCI Free Tier ARM VM has enough resources for a single voice connection and Opus encoding.

## Consequences

Positive:
- Bot gains music playback capability
- Container remains self-contained and legal-risk low
- Implementation scope is bounded

Negative:
- Docker image grows due to FFmpeg
- CPU and memory use increase during voice sessions
- Users must provide audio files or stream URLs themselves

## Command Surface (Initial)

The first implementation should include:

- `/join` — join the user’s current voice channel
- `/leave` — leave the voice channel
- `/play <url>` — play a direct HTTP audio stream
- `/play-file` — play an uploaded local audio file
- `/stop` — stop playback
- `/skip` — skip current track if a queue exists
- `/queue` — show queued tracks

Future commands may include volume control and playlist management.

## Required Axes Impact

- **Encapsulation:** Voice logic lives in a dedicated module. Existing text-only commands remain untouched.
- **Separation of concerns:** Audio source handling, voice connection management, and playback queue are separate concerns within the voice module.
- **Invariants:** Only one voice connection per guild. The queue owner enforces ordering and skip semantics.
- **Consistency:** Audio source types are defined once; commands do not special-case sources outside the source abstraction.
- **Fitness for purpose:** HTTP streams plus local files match the concrete need of a small community server without over-building a full media platform.

## Required Design Updates

- Update `README.md` to list voice playback commands once implemented.
- Update `Cargo.toml` dependency notes once Songbird is added.
- Update the containerization implementation plan to include FFmpeg in the Docker image.
