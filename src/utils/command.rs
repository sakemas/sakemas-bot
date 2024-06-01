use poise::{
    serenity_prelude::{self as serenity, EditMessage},
    CreateReply,
};
use serenity::{
    ComponentInteraction, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse,
};
use std::time::Duration;

use crate::{utils::reaction::CustomReaction, Context, Error};

pub enum ConfirmStyle {
    YesNo,
    OkCancel,
    DangerYesNo,
    DangerOkCancel,
}

/// Get a confirmation from the user.
pub async fn get_confirmation_poise(
    ctx: &Context<'_>,
    message: &str,
    style: ConfirmStyle,
) -> Result<bool, Error> {
    let id = ctx.id();

    let reply = {
        let components = get_confirm_components(id, style);

        CreateReply::default()
            .content(message)
            .components(components)
    };

    let sent_reply = ctx.send(reply).await?;

    if let Some(mci) = ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(20))
        .filter(move |mci| mci.data.custom_id.split('_').next().unwrap_or("") == id.to_string())
        .await
    {
        let custom_id_type = mci.data.custom_id.rsplit('_').next().unwrap_or("");
        settle_confirmation_poise(Some((&mci, message)), ctx, sent_reply).await?;

        match custom_id_type {
            "true" => return Ok(true),
            _ => return Ok(false),
        }
    }

    settle_confirmation_poise(None, ctx, sent_reply).await?;
    Ok(false)
}

async fn settle_confirmation_poise(
    mci: Option<(&ComponentInteraction, &str)>,
    ctx: &Context<'_>,
    sent_reply: poise::ReplyHandle<'_>,
) -> Result<(), Error> {
    match mci {
        Some(mci) => {
            mci.0
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;
            sent_reply
                .edit(
                    *ctx,
                    CreateReply::default().content(mci.1).components(Vec::new()),
                )
                .await?
        }
        None => {
            sent_reply
                .edit(
                    *ctx,
                    CreateReply::default()
                        .content("確認がタイムアウトしました。")
                        .components(Vec::new()),
                )
                .await?
        }
    }
    Ok(())
}

pub async fn get_confirmation_serenity(
    ctx: &serenity::Context,
    message: &serenity::Message,
    confirm_message: &str,
    style: ConfirmStyle,
) -> Result<(bool, serenity::Message), Error> {
    let mut reply = message.reply(&ctx.http, confirm_message).await.unwrap();
    reply
        .edit(
            &ctx.http,
            EditMessage::new().components(get_confirm_components(reply.id.into(), style)),
        )
        .await
        .unwrap();

    let id = reply.id;

    if let Some(mci) = ComponentInteractionCollector::new(ctx)
        .author_id(message.author.id)
        .channel_id(message.channel_id)
        .timeout(Duration::from_secs(20))
        .filter(move |mci| mci.data.custom_id.split('_').next().unwrap_or("") == id.to_string())
        .await
    {
        let custom_id_type = mci.data.custom_id.rsplit('_').next().unwrap_or("");
        settle_confirmation_serenity(Some((&mci, confirm_message)), &ctx.http, &mut reply).await?;

        match custom_id_type {
            "true" => return Ok((true, reply)),
            _ => return Ok((false, reply)),
        }
    }

    settle_confirmation_serenity(None, &ctx.http, &mut reply).await?;
    Ok((false, reply))
}

async fn settle_confirmation_serenity(
    mci: Option<(&ComponentInteraction, &str)>,
    http: &serenity::Http,
    sent_reply: &mut serenity::Message,
) -> Result<(), Error> {
    match mci {
        Some(mci) => {
            mci.0
                .create_response(http, CreateInteractionResponse::Acknowledge)
                .await?;
            sent_reply
                .edit(
                    http,
                    EditMessage::new().content(mci.1).components(Vec::new()),
                )
                .await?
        }
        None => {
            sent_reply
                .edit(
                    http,
                    EditMessage::new()
                        .content("確認がタイムアウトしました。")
                        .components(Vec::new()),
                )
                .await?
        }
    }
    Ok(())
}

fn get_confirm_components(id: u64, style: ConfirmStyle) -> Vec<CreateActionRow> {
    vec![CreateActionRow::Buttons(match style {
        ConfirmStyle::YesNo => {
            vec![
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("はい")
                    .emoji(CustomReaction::Maru.reaction_type())
                    .custom_id(format!("{id}_true")),
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Secondary)
                    .label("いいえ")
                    .emoji(CustomReaction::Batsu.reaction_type())
                    .custom_id(format!("{id}_false")),
            ]
        }
        ConfirmStyle::OkCancel => {
            vec![
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("OK")
                    .emoji(CustomReaction::Maru.reaction_type())
                    .custom_id(format!("{id}_true")),
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Secondary)
                    .label("キャンセル")
                    .emoji(CustomReaction::Batsu.reaction_type())
                    .custom_id(format!("{id}_false")),
            ]
        }
        ConfirmStyle::DangerYesNo => {
            vec![
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Danger)
                    .label("はい")
                    .emoji(CustomReaction::Maru.reaction_type())
                    .custom_id(format!("{id}_true")),
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("いいえ")
                    .emoji(CustomReaction::Batsu.reaction_type())
                    .custom_id(format!("{id}_false")),
            ]
        }
        ConfirmStyle::DangerOkCancel => {
            vec![
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Danger)
                    .label("OK")
                    .emoji(CustomReaction::Maru.reaction_type())
                    .custom_id(format!("{id}_true")),
                CreateButton::new(format!("{id}"))
                    .style(serenity::ButtonStyle::Primary)
                    .label("キャンセル")
                    .emoji(CustomReaction::Batsu.reaction_type())
                    .custom_id(format!("{id}_false")),
            ]
        }
    })]
}
