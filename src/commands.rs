use chrono::Utc;
use poise::CreateReply;

use crate::{
    utils::command::{get_confirmation_poise, ConfirmStyle},
    Context, Error,
};

/// ヘルプメニューを表示します
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "ヘルプを表示したいコマンド"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    info!("help command executed");
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "SAKEM@Sのために醸されたbot",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// 管理人のみ: VC呑み告知を取得し、使用回数を増やします。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn get_vc_announcement(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let mut transaction = pool.begin().await?;

    let row: (i32, String) = sqlx::query_as(
        "SELECT announcement_id, content
        FROM announcements
        ORDER BY usage_count ASC
        LIMIT 1",
    )
    .fetch_one(&mut *transaction)
    .await?;

    let (announcement_id, content) = row;

    sqlx::query(
        "UPDATE announcements
         SET usage_count = usage_count + 1
         WHERE announcement_id = $1",
    )
    .bind(announcement_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    ctx.say(content).await?;

    Ok(())
}

/// 管理人のみ: VC呑み告知を追加します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn add_vc_announcement(
    ctx: Context<'_>,
    #[description = "追加する告知"] content: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    sqlx::query(
        "INSERT INTO announcements (content)
        VALUES ($1)",
    )
    .bind(&content)
    .execute(pool)
    .await?;

    ctx.say("追加しました").await?;

    Ok(())
}

/// 管理人のみ: VC呑み告知をすべて削除します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn delete_all_vc_announcements(ctx: Context<'_>) -> Result<(), Error> {
    let mut proceed = get_confirmation_poise(
        &ctx,
        "本当にすべての告知を削除してよろしいですか？",
        ConfirmStyle::DangerOkCancel,
    )
    .await?;

    if proceed {
        proceed =
            get_confirmation_poise(&ctx, "本当の本当に？", ConfirmStyle::DangerOkCancel).await?;
    }

    if proceed {
        let reply = ctx.say("削除中...").await?;

        let pool = &ctx.data().pool;

        sqlx::query("DELETE FROM announcements")
            .execute(pool)
            .await?;

        reply
            .edit(ctx, CreateReply::default().content("削除しました"))
            .await?;
    } else {
        ctx.say("キャンセルしました").await?;
    }

    Ok(())
}

/// 管理人のみ: VC呑み告知のリストを表示します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn list_vc_announcements(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let rows: Vec<(i32, String, i32)> = sqlx::query_as(
        "SELECT announcement_id, content, usage_count
        FROM announcements
        ORDER BY usage_count DESC",
    )
    .fetch_all(pool)
    .await?;

    let mut message = "VC呑み告知リスト:\n".to_string();
    for (i, (_announcement_id, content, usage_count)) in rows.iter().enumerate() {
        message.push_str(&format!("{}. [{}] {}\n", i + 1, usage_count, content));
    }

    ctx.say(message).await?;

    Ok(())
}

/// 管理人のみ: データベースにアイドルを追加します。
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn add_idol(
    ctx: Context<'_>,
    #[description = "追加するアイドルの名前"] name: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    sqlx::query(
        "INSERT INTO idols (name)
        VALUES ($1)",
    )
    .bind(&name)
    .execute(pool)
    .await?;

    ctx.say("追加しました").await?;

    Ok(())
}

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
        .bind(Some(0i32))
        .bind(&access_token)
        .bind(&refresh_token)
        .execute(pool)
        .await?;

        reply
            .edit(ctx, CreateReply::default().content("初期化しました"))
            .await?;
    } else {
        ctx.say("キャンセルしました").await?;
    }

    Ok(())
}
