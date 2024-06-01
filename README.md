# SAKEM@S bot

## 機能

- [x] 新規メンバー加入時のリアクション
- [x] Twitterとの連携
- [ ] 毎週金曜日22時のVC呑み告知機能
- [ ] DMによる告知の保存、キューイング
- [ ] Twitterでの画像投稿

### 要検討

- [ ] VCに入った人に自動的に'ブラジリアン'ロールを付与
- [ ] 寝落ちた人を寝落ちに送る
- [ ] Blueskyとの連携
- [ ] AIによる告知文生成

## 開発

### 環境変数

```toml
# ./Secrets.toml
# and ./Secrets.dev.toml for development

DISCORD_TOKEN = '***'
VC_ANNOUNCEMENT_CHANNEL = '***'
WELCOME_CHANNEL = '***'
CAUTION_CHANNEL = '***'
INTRODUCTION_CHANNEL = '***'
```
