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
    let parse_result =
        twitter_text::parse(&message.content, twitter_text_config::config_v3(), true);

    if parse_result.is_valid {
        if message.channel_id == Channel::XPoster.id() && !message.author.bot {
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

                let token = match get_access_token(data).await {
                    Ok(token) => token,
                    Err(e) => {
                        reply
                            .edit(
                                &ctx.http,
                                EditMessage::new()
                                    .content("トークンの取得に失敗しました。")
                                    .components(Vec::new()),
                            )
                            .await
                            .unwrap();
                        eprintln!("Failed to get access token: {:?}", e);
                        return;
                    }
                };
                twitter::tweet(&token, &message.content, &message.attachments)
                    .await
                    .unwrap();

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
    } else if parse_result.weighted_length > 280 {
        message
            .reply(
                &ctx.http,
                format!(
                    "テキストが長すぎます。\nweighted length: **{}**/280",
                    parse_result.weighted_length
                ),
            )
            .await
            .unwrap();
    } else if parse_result.weighted_length == 0 {
        message
            .reply(&ctx.http, "テキストが空です。")
            .await
            .unwrap();
    } else {
        message
            .reply(&ctx.http, "無効な文字が含まれています。")
            .await
            .unwrap();
    }
}
