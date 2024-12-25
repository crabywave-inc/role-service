use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Member {
    pub user_id: String,
    pub guild_id: String,
    pub role_ids: Vec<String>,
}