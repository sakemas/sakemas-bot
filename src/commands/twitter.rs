use chrono::Utc;
use poise::CreateReply;

use crate::{
    utils::{
        command::{get_confirmation_poise, ConfirmStyle},
        twitter::{delete_post, get_access_token, TwitterError},
    },
    Context, Error,
};

/// 管理人のみ: TwitterのAccess TokenとRefresh Tokenを初期化します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn set_twitter_tokens(
    ctx: Context<'_>,
    #[description = "Access Token"] access_token: String,
    #[description = "Reflesh Token"] refresh_token: String,
) -> Result<(), Error> {
    let proceed = get_confirmation_poise(
        &ctx,
        "入力されたトークンで初期化してよろしいですか？",
        ConfirmStyle::DangerOkCancel,
    )
    .await?;

    // if the user confirmed, update the tokens
    if proceed {
        let reply = ctx.say("初期化中...").await?;

        // update the last refreshed time
        {
            ctx.data()
                .twitter_token_refreshed_at
                .lock()
                .unwrap()
                .replace(Utc::now());
        }

        let pool = &ctx.data().pool;

        sqlx::query(
            "INSERT INTO twitter_tokens (id, expires_in, access_token, refresh_token)
            VALUES (1, $1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
            expires_in = EXCLUDED.expires_in,
            access_token = EXCLUDED.access_token,
            refresh_token = EXCLUDED.refresh_token",
        )
        .bind(Some(0i64))
        .bind(&access_token)
        .bind(&refresh_token)
        .execute(pool)
        .await?;

        reply
            .edit(ctx, CreateReply::default().content("初期化しました。"))
            .await?;
    } else {
        ctx.say("キャンセルしました。").await?;
    }

    Ok(())
}

/// 管理人のみ: 指定されたIDのツイートを削除します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn delete_tweet(
    ctx: Context<'_>,
    #[description = "Tweet ID"] post_id: String,
) -> Result<(), Error> {
    let data = ctx.data();
    let token = get_access_token(data).await?;
    let access_token = match token.access_token.as_ref() {
        Some(token) => token,
        None => return Err(Box::new(TwitterError::Other("Access token not found".to_string()))),
    };

    let proceed = get_confirmation_poise(
        &ctx,
        &format!("以下のポストを削除しますか？\nid: `{post_id}`\nhttps://x.com/sakemasdiscord/status/{post_id}"),
        ConfirmStyle::DangerOkCancel,
    )
    .await?;

    if proceed {
        delete_post(access_token, &post_id).await?;
        ctx.reply("削除しました。").await?;
    } else {
        ctx.reply("キャンセルしました。").await?;
    }

    Ok(())
}
