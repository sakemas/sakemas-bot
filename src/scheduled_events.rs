use crate::utils::channel::Channel;
use chrono::Utc;
use cron::Schedule;
use poise::serenity_prelude::Http;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time;

pub async fn schedule_vc_announcement(http: Arc<Http>, cron: &str) {
    let schedule = Schedule::from_str(cron).unwrap();

    let channel_id = Channel::VcAnnouncement.id();

    let mut now = Utc::now();
    for next_event in schedule.upcoming(Utc).take(10) {
        let delay = next_event - now;
        time::sleep(delay.to_std().unwrap()).await;
        let _ = channel_id.say(&http, "イベント告知メッセージ").await;
        info!("Scheduled event: {:?}", next_event);
        now = Utc::now();
    }
}
