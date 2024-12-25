use crate::domain::member::events::MemberCreatedEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Member {
    #[serde(rename = "member_id")]
    pub id: String,
    pub user_id: String,
    pub guild_id: String,
    pub role_ids: Vec<String>,
}

impl Member {
    pub fn from_event(event: MemberCreatedEvent) -> Self {
        Self {
            id: event.member_id,
            user_id: event.user_id,
            guild_id: event.guild_id,
            role_ids: vec![],
        }
    }
}
