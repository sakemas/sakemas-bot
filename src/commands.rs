mod idol;
mod twitter;
mod vc_announcement;

// pub use idol::add_idol;
pub use twitter::{
    delete_tweet, set_twitter_tokens,
};
pub use vc_announcement::{
    add_vc_announcement, delete_all_vc_announcements, get_vc_announcement, list_vc_announcements
};

/*
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
*/
