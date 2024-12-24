use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub server_id: String,
}
