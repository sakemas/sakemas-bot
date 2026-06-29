# Deployment Guide

This document describes how to deploy SAKEM@S bot to Oracle Cloud Infrastructure (OCI) Free Tier.

## Prerequisites

- An OCI account with Always Free tier eligibility.
- A local machine with Docker and Docker Compose installed.
- The repository cloned locally and `docker compose build` verified.

## 1. Provision the OCI VM

1. Open the OCI Console and navigate to **Compute → Instances**.
2. Click **Create Instance**.
3. Choose the target compartment and give the instance a name (e.g., `sakemas-bot`).
4. Under **Placement**, select the **Tokyo** availability domain if available; otherwise use Osaka.
5. Under **Image and Shape**:
   - Image: **Oracle Linux 9** or **Canonical Ubuntu 22.04**.
   - Shape: **VM.Standard.A1.Flex** (ARM).
   - OCPUs: **1**.
   - Memory: **4 GB**.
6. Under **Networking**:
   - Create or select a VCN and public subnet.
   - Select **Assign public IPv4 address**.
7. Under **Add SSH keys**, generate or upload an SSH key pair.
8. Click **Create**.

## 2. Configure the Security List

1. Navigate to the subnet’s **Security List**.
2. Add an ingress rule for SSH:
   - Source Type: **CIDR**
   - Source CIDR: your local IP address with `/32` (e.g., `203.0.113.10/32`)
   - Protocol: **TCP**
   - Destination Port Range: **22**
3. No inbound rules are required for the Discord bot because it connects outbound.

## 3. Install Docker on the VM

SSH into the VM and run:

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

For Oracle Linux, use the official Docker installation steps for RHEL-based distributions instead.

## 4. Transfer the Project

From your local machine, run:

```bash
scp -i ~/.ssh/oci_key .env oracle-cloud@INSTANCE_IP:/home/oracle-cloud/sakemas-bot.env

# Copy project files excluding .env and target/
rsync -avz --exclude=.env --exclude=target --exclude=.git . \
  -e "ssh -i ~/.ssh/oci_key" \
  oracle-cloud@INSTANCE_IP:/home/oracle-cloud/sakemas-bot/
```

## 5. Deploy

SSH into the VM and run:

```bash
cd /home/oracle-cloud/sakemas-bot
mv /home/oracle-cloud/sakemas-bot.env .env
docker compose up -d
```

Verify:

```bash
docker compose ps
docker compose logs -f app
```

## 6. Cutover from Shuttle

1. Verify that the bot is online in Discord and responds to slash commands.
2. In the Discord Developer Portal, consider regenerating the bot token if the old one was ever at risk.
3. Stop the Shuttle deployment:

```bash
cargo shuttle stop
```

4. Confirm that only the OCI instance remains online.

## 7. Operational Notes

- Backups: schedule `pg_dump` of the PostgreSQL volume or snapshot the OCI boot volume.
- Updates: pull the latest code, rebuild the image, and run `docker compose up -d --build`.
- Logs: use `docker compose logs -f app` for live logs.
