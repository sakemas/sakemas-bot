use crate::utils::channel::get_channel_id;
use poise::serenity_prelude::{
    model::{guild::Member, mention::Mention},
    Context,
};

/// Send a welcome message to the welcome channel when a new member joins.
pub async fn addition(ctx: &Context, new_member: &Member) {
    let post_channel = get_channel_id("WELCOME_CHANNEL");
    let caution_channel = get_channel_id("CAUTION_CHANNEL");
    let introduction_channel = get_channel_id("INTRODUCTION_CHANNEL");

    let mention = Mention::from(new_member.user.id);
    let caution_channel = Mention::from(caution_channel);
    let introduction_channel = Mention::from(introduction_channel);

    let message = format!(
        "{}さん、アイマスとお酒のDiscord、SAKEM@Sへようこそ。\n{}をご一読の上、ぜひ{}をお願いします！",
        mention, caution_channel, introduction_channel
    );

    let _ = post_channel.say(&ctx.http, message).await;
}
