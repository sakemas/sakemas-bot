use poise::serenity_prelude::model::guild::Member;

/// Get a reference to the name of a member.
///
/// If the member has a nickname, return a reference to the nickname.
///
/// If the member has no nickname and a global name, return a reference to the global name.
///
/// Othewise, return a reference to the username.
pub fn get_name_ref(new_member: &Member) -> &String {
    match new_member.nick {
        Some(ref nick) => nick,
        None => match new_member.user.global_name {
            Some(ref name) => name,
            None => &new_member.user.name,
        },
    }
}
