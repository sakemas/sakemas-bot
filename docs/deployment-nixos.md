# NixOS Deployment Guide

> This guide is a **prototype** for the NixOS deployment path. It has not been
> executed against a live Oracle Cloud instance yet. Validate each step on a
> test VM before relying on it for production.

## Overview

This deployment produces a NixOS qcow2 boot image containing:

- the SAKEM@S bot package,
- PostgreSQL as a NixOS service,
- a systemd unit for the bot,
- SSH access and a basic firewall.

The image is built on an `aarch64-linux` host (for example, a temporary Oracle
Cloud ARM VM), uploaded to OCI as a custom image, and used to launch the
production VM.

## Prerequisites

- An OCI Free Tier account in the Tokyo region.
- An `aarch64-linux` machine with Nix installed, or an OCI ARM VM provisioned
  for the build.
- This repository cloned on the build host.
- A `.env` / `secrets.env` file ready (see [Secrets](#secrets)).

## Build the OCI image

On the `aarch64-linux` build host:

```bash
nix build .#packages.aarch64-linux.oci-image
```

The output is a symbolic link `result` pointing to a directory that contains a
`nixos.qcow2` file.

> macOS cannot build this output directly because `oci-image.nix` creates a
> full Linux boot disk. Either build on an ARM VM, use an `aarch64-linux` remote
> builder, or use QEMU emulation (slow).

## Upload the image to OCI

1. Create an OCI Object Storage bucket (for example, `sakemas-images`).
2. Upload `result/nixos.qcow2` to the bucket.
3. In the OCI Console, go to **Compute > Custom Images > Import Image**.
4. Choose the uploaded object and select **QCOW2** format.
5. Wait for the import to complete.

## Launch the VM

1. Create a VM from the imported custom image.
2. Choose an ARM shape (for example, VM.Standard.A1.Flex).
3. Add an SSH public key. The NixOS image fetches authorized keys from the OCI
   instance metadata service on first boot.
4. Assign a public IP or use a bastion host.
5. Open port 22 in the security list.

## First boot

SSH into the VM as `root`:

```bash
ssh root@<instance-ip>
```

If the authorized keys were not fetched automatically, connect via the OCI
serial console and add a key manually.

## Secrets

The bot expects environment variables. Copy your local secrets file to the VM:

```bash
scp /path/to/secrets.env root@<instance-ip>:/etc/sakemas-bot/secrets.env
ssh root@<instance-ip> 'chmod 640 /etc/sakemas-bot/secrets.env && chown root:sakemas-bot /etc/sakemas-bot/secrets.env'
```

The file must be in **systemd EnvironmentFile format** (`KEY=VALUE`, one per
line). Example:

```env
DISCORD_TOKEN=...
DATABASE_URL=postgres://sakemas_bot:your_password@localhost/sakemas_bot
VC_ANNOUNCEMENT_CHANNEL=...
WELCOME_CHANNEL=...
CAUTION_CHANNEL=...
INTRODUCTION_CHANNEL=...
X_POSTER_CHANNEL=...
TWITTER_CLIENT_ID=...
TWITTER_CLIENT_SECRET=...
```

> Do not use TOML-style quotes. systemd parses this file literally, so quotes
> become part of the value.

## PostgreSQL password

The NixOS module creates the `sakemas_bot` database and user automatically, but
the user's password must be set manually on first boot:

```bash
sudo -u postgres psql -c "ALTER USER sakemas_bot WITH PASSWORD 'your_password';"
```

Make sure the password matches the one in `/etc/sakemas-bot/secrets.env`.

## Start and check the bot

The systemd service is enabled automatically. After placing the secrets file,
start it:

```bash
systemctl start sakemas-bot
systemctl status sakemas-bot
journalctl -u sakemas-bot -f
```

## Update the deployment

To deploy a new version of the bot:

1. Build a new image on the build host.
2. Upload it to OCI and import it as a new custom image.
3. Terminate the old VM and launch a new one from the new image.
4. Copy `secrets.env` to the new VM and start the service.

Alternatively, once the VM is running, you can use `nixos-rebuild` to update it
in place if the build host is reachable from the VM (remote build or binary
cache). This avoids re-importing images.

## Rollback

NixOS keeps previous system generations. If an update breaks the bot, select a
previous generation in the bootloader or run:

```bash
nixos-rebuild switch --rollback
```

## Backups

PostgreSQL data lives on the VM's root disk. Back up `/var/lib/postgresql`
regularly, either with OCI volume backups or with `pg_dump`.

## Cost

This path uses only the OCI Always Free resources:

- 1 ARM VM (up to 4 OCPU, 24 GB RAM).
- Boot volume (up to 200 GB).
- Object Storage for image uploads (subject to Always Free limits).
