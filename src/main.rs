#[macro_use]
extern crate tracing;

use chrono::{DateTime, Utc};
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

use self::{
    commands::*,
    event_handler::event_handler,
    utils::secret::{get_secret, set_env_var},
};

mod commands;
mod event_handler;
mod scheduled_events;
pub mod utils;

pub struct Data {
    pool: PgPool,
    secret_store: SecretStore,
    twitter_token_refreshed_at: Arc<Mutex<Option<DateTime<Utc>>>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleSerenity {
    sqlx::migrate!().run(&pool).await.unwrap();

    let discord_token = get_secret(&secret_store, "DISCORD_TOKEN")?;

    set_env_var(&secret_store, "TWITTER_CLIENT_ID")?;
    set_env_var(&secret_store, "TWITTER_CLIENT_SECRET")?;

    set_env_var(&secret_store, "VC_ANNOUNCEMENT_CHANNEL")?;
    set_env_var(&secret_store, "WELCOME_CHANNEL")?;
    set_env_var(&secret_store, "CAUTION_CHANNEL")?;
    set_env_var(&secret_store, "INTRODUCTION_CHANNEL")?;
    set_env_var(&secret_store, "X_POSTER_CHANNEL")?;

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    pool,
                    secret_store,
                    twitter_token_refreshed_at: Arc::new(Mutex::new(None)),
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                help(),
                get_vc_announcement(),
                list_vc_announcements(),
                add_vc_announcement(),
                delete_all_vc_announcements(),
                set_twitter_tokens(),
            ],
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
