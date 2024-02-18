use poise::serenity_prelude::model::id::ChannelId;
use std::str::FromStr;

/// Get a channel ID from an environment variable.
pub fn get_channel_id(name: &str) -> ChannelId {
    let channel_id = std::env::var(name).unwrap();
    ChannelId::from_str(&channel_id).unwrap()
}
