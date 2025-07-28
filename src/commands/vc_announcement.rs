use poise::CreateReply;

use crate::{
    utils::command::{get_confirmation_poise, ConfirmStyle},
    Context, Error,
};

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

    ctx.say("追加しました。").await?;

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
            .edit(ctx, CreateReply::default().content("削除しました。"))
            .await?;
    } else {
        ctx.say("キャンセルしました。").await?;
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
