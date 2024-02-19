pub mod channel;
pub mod member;
pub mod secret;

use poise::serenity_prelude::model::mention::Mention;

pub trait Mentionable {
    fn mention(&self) -> Mention;
}
