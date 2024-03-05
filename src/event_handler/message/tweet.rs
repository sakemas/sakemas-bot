use poise::serenity_prelude::{self as serenity, EditMessage};

use serenity::{Context, Message};

use crate::{
    utils::{
        channel::Channel,
        command::{get_confirmation_serenity, ConfirmStyle},
        twitter::{self, get_access_token},
    },
    Data,
};

pub async fn post(ctx: &Context, message: &Message, data: &Data) {
    if message.channel_id == Channel::XPoster.id() && message.author.bot == false {
        let (proceed, mut reply) = get_confirmation_serenity(
            ctx,
            message,
            "この内容でポストしてよろしいですか？",
            ConfirmStyle::OkCancel,
        )
        .await
        .unwrap();

        if proceed {
            reply
                .edit(
                    &ctx.http,
                    EditMessage::new()
                        .content("ポスト中...")
                        .components(Vec::new()),
                )
                .await
                .unwrap();

            let token = get_access_token(data).await.unwrap();
            twitter::tweet(&token, &message.content).await.unwrap();

            reply
                .edit(&ctx.http, EditMessage::new().content("ポストしました。"))
                .await
                .unwrap();
        } else {
            reply
                .edit(
                    &ctx.http,
                    EditMessage::new()
                        .content("ポストをキャンセルしました。")
                        .components(Vec::new()),
                )
                .await
                .unwrap();
        }
    }
}
