use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberCreatedEvent {
    pub member_id: String,
    pub guild_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberRoleAddedEvent {
    pub member_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberRoleRemovedEvent {
    pub member_id: String,
    pub role_id: String,
}
