use poise::serenity_prelude::{model::id::ChannelId, Context};
use std::str::FromStr;
use std::sync::Arc;

pub async fn announce_vc(ctx: Arc<Context>) {
    let channel_id = std::env::var("TEST_VC_ANNOUNCEMENT_CHANNEL")
        .expect("Expected TEST_VC_ANNOUNCEMENT_CHANNEL environment variable");
    let channel_id = ChannelId::from_str(&channel_id).expect("Invalid ChannelId");
    let _ = channel_id.say(&ctx.http, "イベント告知メッセージ").await;
}

pub async fn test_cron(ctx: Arc<Context>) {
    let channel_id = std::env::var("TEST_VC_ANNOUNCEMENT_CHANNEL")
        .expect("Expected TEST_VC_ANNOUNCEMENT_CHANNEL environment variable");
    let channel_id = ChannelId::from_str(&channel_id).expect("Invalid ChannelId");
    let _ = channel_id.say(&ctx.http, "cron test message").await;
}
