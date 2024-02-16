use chrono::Utc;
use cron::Schedule;
use poise::serenity_prelude::{model::id::ChannelId, Context};
use std::str::FromStr;
use std::sync::Arc;
use tokio::time;

pub async fn schedule_vc_announcement(ctx: Arc<Context>, cron: &str) {
    let schedule = Schedule::from_str(cron).expect("Invalid cron expression");

    let mut now = Utc::now();
    for next_event in schedule.upcoming(Utc).take(10) {
        let delay = next_event - now;
        time::sleep(delay.to_std().expect("Invalid Duration")).await;
        announce_vc(&ctx).await;
        println!("VC announcement complete");
        now = Utc::now();
    }
}

pub async fn announce_vc(ctx: &Context) {
    let channel_id = std::env::var("TEST_VC_ANNOUNCEMENT_CHANNEL")
        .expect("Expected TEST_VC_ANNOUNCEMENT_CHANNEL environment variable");
    let channel_id = ChannelId::from_str(&channel_id).expect("Invalid ChannelId");
    let _ = channel_id.say(&ctx.http, "イベント告知メッセージ").await;
}
