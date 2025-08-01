use crate::utils::channel::Channel;
use chrono::Utc;
use cron::Schedule;
use poise::serenity_prelude::Http;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time;

const FRIDAY_NIGHT: &str = "0 0 13 * * Fri *";

pub async fn schedule_vc_announcement(http: Arc<Http>) {
    let schedule = Schedule::from_str(FRIDAY_NIGHT).unwrap();

    let channel_id = Channel::VcAnnouncement.id();

    let mut now = Utc::now();
    for next_event in schedule.upcoming(Utc).take(10) {
        let delay = next_event - now;
        time::sleep(delay.to_std().unwrap()).await;
        let _ = channel_id.say(&http, "@everyone 金曜日 22時定例、VC呑みの時間です！").await;
        info!("Scheduled event: {:?}", next_event);
        now = Utc::now();
    }
}
