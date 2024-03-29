use poise::serenity_prelude::{
    model::{guild::Member, mention::Mention},
    Context,
};

use crate::utils::{channel::Channel, Mentionable};

/// Send a welcome message to the welcome channel when a new member joins.
pub async fn addition(ctx: &Context, new_member: &Member) {
    info!("New member joined: {}", new_member.user.name);
    let post_channel = Channel::Welcome.id();

    let mention = Mention::from(new_member.user.id);

    let message = format!(
        "{}さん、アイマスとお酒のDiscord、SAKEM@Sへようこそ。\n{}をご一読の上、ぜひ{}をお願いします！",
        mention, Channel::Caution.mention(), Channel::Introduction.mention()
    );

    let _ = post_channel.say(&ctx.http, message).await;
}
