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
    if message.channel_id == Channel::XPoster.id() && !message.author.bot {
        let parse_result =
            twitter_text::parse(&message.content, twitter_text_config::config_v3(), true);

        if parse_result.is_valid {
            let (proceed, mut reply) = get_confirmation_serenity(
                ctx,
                message,
                format!(
                    "この内容でポストしてよろしいですか？\n`weighted length: {}/280`",
                    parse_result.weighted_length
                )
                .as_str(),
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
                let result = twitter::tweet(&token, &message.content, &message.attachments).await;

                match result {
                    Ok(result) => {
                        if let Some(data) = result.data {
                            let id = data.id.clone();
                            let message = match id {
                                Some(id) => format!("ポストしました。\n`id: {}`\nhttps://x.com/sakemasdiscord/status/{}", id, id),
                                None => "ポストしました。".to_string(),
                            };

                            reply
                                .edit(&ctx.http, EditMessage::new().content(message))
                                .await
                                .unwrap();

                            eprintln!("Tweet successfully\n{:?}", data);
                        } else {
                            reply
                                .edit(
                                    &ctx.http,
                                    EditMessage::new()
                                        .content(format!("ポストに失敗しました。\n```title: {:?}\nresult_type: {:?}\nstatus: {:?}\ndetail: {:?}```", result.title, result.result_type, result.status, result.detail))
                                        .components(Vec::new()),
                                )
                                .await
                                .unwrap();
                        }
                    }
                    Err(e) => {
                        reply
                            .edit(
                                &ctx.http,
                                EditMessage::new()
                                    .content(format!("ポストに失敗しました。\n```{:?}```", e))
                                    .components(Vec::new()),
                            )
                            .await
                            .unwrap();
                    }
                }
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
        } else if parse_result.weighted_length > 280 {
            message
                .reply(
                    &ctx.http,
                    format!(
                        "テキストが長すぎます。\n`weighted length: {}/280`",
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
}
