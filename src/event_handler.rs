use crate::scheduled_events::schedule_vc_announcement;
use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);

            tokio::spawn(schedule_vc_announcement(
                Arc::new(ctx.clone()),
                "0 0 13 * * Fri *",
            ));
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().contains("poise") {
                let mentions = data.poise_mentions.load(Ordering::SeqCst) + 1;
                data.poise_mentions.store(mentions, Ordering::SeqCst);
                new_message
                    .reply(ctx, format!("Poise has been mentioned {} times", mentions))
                    .await?;
            }
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {}
        _ => {}
    }
    Ok(())
}
