use poise::serenity_prelude::model::mention::Mention;

pub mod channel;
pub mod command;
pub mod emoji;
pub mod member;
pub mod reaction;
pub mod secret;

pub trait Mentionable {
    fn mention(&self) -> Mention;
}
