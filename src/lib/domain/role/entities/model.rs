use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub server_id: String,
    pub color: String,
    pub position: i32,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permissions: String,
    pub color: String,
    pub hoist: bool,
    pub mentionable: bool,
}
