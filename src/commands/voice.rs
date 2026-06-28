use poise::serenity_prelude::Attachment;
use songbird::input::{HttpRequest, Input};

use crate::{Context, Error};

/// VCに参加します。
#[poise::command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("このコマンドはサーバー内でのみ使用できます")?;

    let channel_id = ctx
        .guild()
        .and_then(|guild| {
            guild
                .voice_states
                .get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id)
        })
        .ok_or("あなたが参加しているVCが見つかりません")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird manager が見つかりません")?;

    let _call = manager
        .join(guild_id, channel_id)
        .await
        .map_err(|e| format!("VCへの参加に失敗しました: {e}"))?;

    ctx.say("VCに参加しました。").await?;
    Ok(())
}

/// VCから退出します。
#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("このコマンドはサーバー内でのみ使用できます")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird manager が見つかりません")?;

    manager
        .remove(guild_id)
        .await
        .map_err(|e| format!("退出に失敗しました: {e}"))?;

    ctx.say("VCから退出しました。").await?;
    Ok(())
}

/// URLから音声を再生します（HTTPストリームのみ対応）。
#[poise::command(slash_command)]
pub async fn play_url(
    ctx: Context<'_>,
    #[description = "再生する音声のURL"] url: String,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("このコマンドはサーバー内でのみ使用できます")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird manager が見つかりません")?;

    let handler = manager
        .get(guild_id)
        .ok_or("VCに参加していません。/join を先に実行してください")?;

    let input = Input::from(HttpRequest::new(reqwest::Client::new(), url));

    let mut handler = handler.lock().await;
    handler.enqueue_input(input).await;

    ctx.say("キューに追加しました。").await?;
    Ok(())
}

/// 添付ファイルから音声を再生します。
#[poise::command(slash_command)]
pub async fn play_file(
    ctx: Context<'_>,
    #[description = "再生する音声ファイル"] attachment: Attachment,
) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("このコマンドはサーバー内でのみ使用できます")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird manager が見つかりません")?;

    let handler = manager
        .get(guild_id)
        .ok_or("VCに参加していません。/join を先に実行してください")?;

    let data = attachment.download().await?;
    let input = Input::from(songbird::input::RawAdapter::new(
        std::io::Cursor::new(data),
        48_000,
        2,
    ));

    let mut handler = handler.lock().await;
    handler.enqueue_input(input).await;

    ctx.say("キューに追加しました。").await?;
    Ok(())
}

/// 再生を停止します。
#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx
        .guild_id()
        .ok_or("このコマンドはサーバー内でのみ使用できます")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird manager が見つかりません")?;

    let handler = manager.get(guild_id).ok_or("VCに参加していません")?;

    let mut handler = handler.lock().await;
    handler.stop();

    ctx.say("再生を停止しました。").await?;
    Ok(())
}
