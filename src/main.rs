#[macro_use]
extern crate tracing;

use anyhow::Context as _;
use chrono::{DateTime, Utc};
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

use self::{commands::*, event_handler::event_handler, utils::secret::get_secret};

mod commands;
mod event_handler;
mod scheduled_events;
pub mod utils;

pub struct Data {
    pool: PgPool,
    twitter_client_id: String,
    twitter_client_secret: String,
    twitter_token_refreshed_at: Arc<Mutex<Option<DateTime<Utc>>>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let database_url = get_secret("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url)
        .await
        .context("failed to connect to database")?;

    sqlx::migrate!().run(&pool).await.unwrap();

    let discord_token = get_secret("DISCORD_TOKEN")?;
    let twitter_client_id = get_secret("TWITTER_CLIENT_ID")?;
    let twitter_client_secret = get_secret("TWITTER_CLIENT_SECRET")?;

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    pool,
                    twitter_client_id,
                    twitter_client_secret,
                    twitter_token_refreshed_at: Arc::new(Mutex::new(None)),
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                // VC Announcement
                get_vc_announcement(),
                list_vc_announcements(),
                add_vc_announcement(),
                delete_all_vc_announcements(),
                // Twitter
                delete_tweet(),
                set_twitter_tokens(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let mut client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .context("failed to build Discord client")?;

    client.start().await.context("client exited with error")?;
    Ok(())
}
