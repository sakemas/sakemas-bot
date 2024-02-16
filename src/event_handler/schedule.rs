use chrono::Utc;
use cron::Schedule;
use poise::serenity_prelude::Context;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time;

pub async fn schedule<T>(ctx: Arc<Context>, cron: &str, func: impl Fn(Arc<Context>) -> T)
where
    T: Future<Output = ()> + Send + 'static,
{
    let schedule = Schedule::from_str(cron).expect("Invalid cron expression");
    let ctx = Arc::clone(&ctx);

    let mut now = Utc::now();
    for next_event in schedule.upcoming(Utc).take(10) {
        let delay = next_event - now;
        time::sleep(delay.to_std().expect("Invalid Duration")).await;
        func(Arc::clone(&ctx));
        now = Utc::now();
    }
}
