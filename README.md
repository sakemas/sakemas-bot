# SAKEM@S bot

Discordサーバー "**SAKEM@S**" 専用bot。

_main_ ブランチへの merge により、Northflank 上で自動的にデプロイされる予定です。

## デプロイ

本番デプロイ手順は `docs/deployment-northflank.md` を参照してください。これは現在の試行構成です。

OCI Free Tier 向けの Docker Compose 手順は `docs/deployment.md`、NixOS 向けの手順は `docs/deployment-nixos.md` に残っています。

実行環境については `design/adr/0001-runtime-environment.md`、現在の試行については `design/adr/0007-northflank-sandbox-feasibility.md` を参照してください。

### Northflank Sandbox 試行中

現在、Northflank Sandbox を使った無料・常時稼働の PaaS デプロイを検証しています。

- 調査結果: `design/adr/0007-northflank-sandbox-feasibility.md`
- 手順: `docs/deployment-northflank.md`

OCI Free Tier の ARM キャパシティが確保できない間、Northflank での動作確認を進めます。

## 機能

- [x] 新規メンバー加入時のリアクション
- [x] Twitterとの連携
- [x] Twitterでの画像投稿
- [ ] 毎週金曜日22時のVC呑み告知機能
- [ ] DMによる告知の保存、キューイング

### 要検討

- [ ] VCに入った人に自動的に'ブラジリアン'ロールを付与
- [ ] 寝落ちた人を寝落ちに送る
- [ ] Blueskyとの連携
- [ ] AIによる告知文生成

## 設計

アーキテクチャ決定は `design/adr/` に記録されています。

## 開発

### 環境変数

1. `.env.example` を `.env` にコピーします。
2. 開発時は `Secrets.dev.toml` の値を `.env` に移行します。`Secrets.dev.toml` は `.gitignore` 対象ですが、移行後は作業ツリーから削除してください。
3. 本番値は Northflank ダッシュボードの環境変数 / Secret groups に設定します。OCI VM を使う場合は VM 上の `.env` に直接設定します。

```bash
cp .env.example .env
```

```env
# ./.env for development

DATABASE_URL='postgresql://postgres:password@localhost:5432/sakemas_bot'
DISCORD_TOKEN='***'
TWITTER_CLIENT_ID='***'
TWITTER_CLIENT_SECRET='***'
VC_ANNOUNCEMENT_CHANNEL='***'
WELCOME_CHANNEL='***'
CAUTION_CHANNEL='***'
INTRODUCTION_CHANNEL='***'
X_POSTER_CHANNEL='***'
```

### Docker Compose

ローカル開発用の Docker Compose 構成です。本番の Northflank デプロイとは別物です。

```bash
docker compose up -d db
```

ビルド確認：

```bash
docker compose build
```

本番同等の構成で bot も起動する場合は、`.env` を設定した上で：

```bash
docker compose up -d
```

ただし、同じ `DISCORD_TOKEN` を使った bot が複数起動すると Discord Gateway で競合するため、本番移行完了までは app サービスをローカルで常時起動しないでください。現在 Northflank 上の本番サービスとの競合を避けるため、`.env` の `DISCORD_TOKEN` を使用する前に必ず確認してください。
