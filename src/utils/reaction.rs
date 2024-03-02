use poise::serenity_prelude::ReactionType;

use crate::utils::emoji::CustomEmoji;

pub enum CustomReaction {
    Maru,
    Batsu,
}

impl CustomReaction {
    pub fn reaction_type(&self) -> ReactionType {
        match self {
            CustomReaction::Maru => ReactionType::Custom {
                animated: false,
                id: CustomEmoji::Maru.id(),
                name: Some("まる".to_string()),
            },
            CustomReaction::Batsu => ReactionType::Custom {
                animated: false,
                id: CustomEmoji::Batsu.id(),
                name: Some("ばつ".to_string()),
            },
        }
    }
}
