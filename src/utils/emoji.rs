use poise::serenity_prelude::EmojiId;

pub enum CustomEmoji {
    Maru,
    Batsu,
}

impl CustomEmoji {
    pub fn id(&self) -> EmojiId {
        match self {
            CustomEmoji::Maru => EmojiId::new(873697118940958750),
            CustomEmoji::Batsu => EmojiId::new(874941925172584468),
        }
    }
}
