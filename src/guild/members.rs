use serde::{Deserialize, Serialize};
use serenity::model::guild::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// A member joined the guild
    Addition(Member),
    /// A member left or was removed from the guild
    Removal(Member),
    /// A member's details were updated
    Update(Member),

    /// The member linked a steam account
    SteamLinked(Member, String),
}
