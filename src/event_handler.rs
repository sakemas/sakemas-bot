use poise::serenity_prelude as serenity;

use crate::scheduled_events::schedule_vc_announcement;
use crate::{Data, Error};

mod guild;
mod message;

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
                ctx.http.clone(),
                "0 0 13 * * Fri *",
            ));
        }
        serenity::FullEvent::Message { new_message } => {
            message::tweet::post(ctx, new_message, data).await;
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            guild::member::addition(ctx, new_member).await;
        }
        _ => {}
    }
    Ok(())
}
