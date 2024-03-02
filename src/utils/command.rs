use poise::{serenity_prelude as serenity, CreateReply};
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
pub async fn get_confirmation(
    ctx: &Context<'_>,
    message: &str,
    style: ConfirmStyle,
) -> Result<bool, Error> {
    let id = ctx.id();

    let reply = {
        let components = vec![CreateActionRow::Buttons(match style {
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
        })];

        CreateReply::default()
            .content(message)
            .components(components)
    };

    let sent_reply = ctx.send(reply).await?;

    if let Some(mci) = ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(Duration::from_secs(20))
        .filter(move |mci| mci.data.custom_id.splitn(2, '_').next().unwrap_or("") == id.to_string())
        .await
    {
        let custom_id_type = mci.data.custom_id.rsplitn(2, '_').next().unwrap_or("");
        settle_confirmation(Some((&mci, message)), ctx, sent_reply).await?;

        match custom_id_type {
            "true" => return Ok(true),
            _ => return Ok(false),
        }
    }

    settle_confirmation(None, ctx, sent_reply).await?;
    Ok(false)
}

async fn settle_confirmation(
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
