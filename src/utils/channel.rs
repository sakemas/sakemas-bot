use poise::serenity_prelude::model::{id::ChannelId, mention::Mention};

use super::Mentionable;

/// SAKEM@Sのチャンネル
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    /// ようこそ
    Welcome,
    /// 注意事項
    Caution,
    /// 自己紹介
    Introduction,
    /// VC呑み会の予定
    VcAnnouncement,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match self {
            Channel::Welcome => "WELCOME_CHANNEL",
            Channel::Caution => "CAUTION_CHANNEL",
            Channel::Introduction => "INTRODUCTION_CHANNEL",
            Channel::VcAnnouncement => "VC_ANNOUNCEMENT_CHANNEL",
        };
        write!(f, "{}", id)
    }
}

impl From<Channel> for ChannelId {
    fn from(channel: Channel) -> ChannelId {
        channel.into_id()
    }
}

impl Mentionable for Channel {
    /// Get the mention of the channel.
    fn mention(&self) -> Mention {
        Mention::from(self.into_id())
    }
}

impl Channel {
    /// Get the channel ID of the channel.
    pub fn into_id(&self) -> ChannelId {
        get_channel_id(self.to_string())
    }
}

/// Get a channel ID from an environment variable.
pub fn get_channel_id(name: String) -> ChannelId {
    use std::str::FromStr;
    let channel_id = std::env::var(name).unwrap();
    ChannelId::from_str(&channel_id).unwrap()
}
