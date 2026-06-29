# SAKEM@S bot

Discordサーバー "**SAKEM@S**" 専用bot。

*main*ブランチへのmergeにより、自動的にデプロイされる予定です。

## デプロイ

本番デプロイ手順は `docs/deployment.md` を参照してください。

実行環境については `design/adr/0001-runtime-environment.md` を参照してください。

### NixOS 評価中

NixOS による宣言的デプロイも検討中です。現時点では Docker Compose が採用済みの構成です。

- 調査結果: `design/adr/0006-nixos-deployment.md`
- 実験的な手順: `docs/deployment-nixos.md`

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
3. 本番値は OCI VM 上の `.env` に直接設定します。

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

ローカルで PostgreSQL を立ち上げて動作確認する場合：

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

ただし、同じ `DISCORD_TOKEN` を使った bot が複数起動すると Discord Gateway で競合するため、本番移行完了までは app サービスをローカルで常時起動しないでください。
