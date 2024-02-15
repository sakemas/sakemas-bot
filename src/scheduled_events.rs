use chrono::Utc;
use cron::Schedule;
use poise::serenity_prelude::{model::id::ChannelId, Context};
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::Instant;

pub async fn schedule_vc_announcement(ctx: Arc<Context>) {
    // Define the cron schedule for "Every Friday at 13:00 UST" in cron format
    let schedule = Schedule::from_str("0 0 13 ? * Fri *").unwrap();

    // Calculate the time until the first event from the current time
    let now = Utc::now();
    if let Some(next_event) = schedule.after(&now).next() {
        let delay = next_event - now; // Duration
                                      // Wait until the first event
        tokio::time::sleep_until(Instant::now() + delay.to_std().unwrap()).await;

        // Execute announce_event every time there is a next event
        for datetime in schedule.after(&Utc::now()) {
            announce_vc(&ctx).await;
            let until_next_event = datetime - Utc::now();
            tokio::time::sleep_until(Instant::now() + until_next_event.to_std().unwrap()).await;
        }
    }
}

async fn announce_vc(ctx: &Context) {
    let channel_id = std::env::var("TEST_VC_ANNOUNCEMENT_CHANNEL").unwrap();
    let channel_id = ChannelId::from_str(channel_id.as_str()).unwrap();
    let _ = channel_id.say(&ctx.http, "イベント告知メッセージ").await;
}
