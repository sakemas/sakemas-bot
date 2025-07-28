use crate::{Context, Error};

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

    ctx.say("追加しました。").await?;

    Ok(())
}
