use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MemberCreatedEvent {
    pub member_id: String,
    pub guild_id: String,
    pub user_id: String,
}