mod commands;
mod event_handler;
mod scheduled_events;

use crate::{commands::hello, event_handler::event_handler};
use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::sync::atomic::AtomicU32;

pub struct Data {
    poise_mentions: AtomicU32,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Retrieve the announcement channel ID for test voice channels from the secret store
    // and set it as an environment variable for later use
    let test_vc_announcement_channel = secret_store
        .get("TEST_VC_ANNOUNCEMENT_CHANNEL")
        .context("'TEST_VC_ANNOUNCEMENT_CHANNEL' was not found")?;
    std::env::set_var("TEST_VC_ANNOUNCEMENT_CHANNEL", test_vc_announcement_channel);

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    poise_mentions: AtomicU32::new(0),
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![hello()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
