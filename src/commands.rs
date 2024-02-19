use crate::{Context, Error};
use tracing::info;

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

/// Responds with "world!"
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}
