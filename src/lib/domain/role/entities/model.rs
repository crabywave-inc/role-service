use crate::domain::role::entities::permissions::Permissions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub guild_id: String,
    pub color: String,
    pub position: i32,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}

impl Role {
    pub fn new(id: &str, name: &str, guild_id: &str, color: &str, position: i32) -> Self {
        Role {
            id: id.to_string(),
            name: name.to_string(),
            guild_id: guild_id.to_string(),
            color: color.to_string(),
            position,
            permissions: "0".to_string(),
            hoist: false,
            mentionable: false,
        }
    }

    pub fn from_request(
        request: CreateRoleRequest,
        guild_id: &str,
        position: i32,
    ) -> Result<Self, String> {
        let permissions = match request.permissions.parse::<u64>() {
            Ok(value) => value,
            Err(_) => return Err("Invalid permissions format".to_string()),
        };

        Ok(Role {
            id: uuid::Uuid::new_v4().to_string(), // Génère un UUID unique
            name: request.name,
            guild_id: guild_id.to_string(),
            color: request.color,
            position,
            permissions: format!("{:x}", permissions), // Stocke sous forme hexadécimale
            hoist: request.hoist,
            mentionable: request.mentionable,
        })
    }

    pub fn add_permission(&mut self, permission: Permissions) {
        let current_permissions = u64::from_str_radix(&self.permissions, 16).unwrap_or(0);
        self.permissions = format!("{:x}", current_permissions | (permission as u64));
    }

    pub fn get_permissions(&self) -> Vec<Permissions> {
        let bitfield = self.permissions.parse::<u64>().unwrap_or(0);
        Permissions::from_bitfield(bitfield)
    }

    pub fn has_permission(&self, permission: Permissions) -> bool {
        let bitfield = u64::from_str_radix(&self.permissions, 16).unwrap_or(0);
        (bitfield & (permission as u64)) == (permission as u64)
    }
}

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permissions: String,
    pub color: String,
    pub hoist: bool,
    pub mentionable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::role::entities::permissions::Permissions;

    #[test]
    fn test_add_permission() {
        let mut role = Role::new("1", "Moderator", "server1", "#00FF00", 2);
        role.add_permission(Permissions::ManageChannels);

        assert!(role.has_permission(Permissions::ManageChannels));
        assert!(!role.has_permission(Permissions::Administrator));
    }

    #[test]
    fn test_multiple_permissions() {
        let mut role = Role::new("1", "Moderator", "server1", "#00FF00", 2);
        role.add_permission(Permissions::ManageChannels);
        role.add_permission(Permissions::SendMessages);

        assert!(role.has_permission(Permissions::ManageChannels));
        assert!(role.has_permission(Permissions::SendMessages));
        assert!(!role.has_permission(Permissions::Administrator));
    }

    #[test]
    fn test_no_permission() {
        let role = Role::new("1", "Member", "server1", "#0000FF", 3);

        assert!(!role.has_permission(Permissions::ManageGuild));
        assert!(!role.has_permission(Permissions::Administrator));
    }

    #[test]
    fn test_permission_overlap() {
        let mut role = Role::new("1", "Administrator", "server1", "#FFAA00", 0);
        role.add_permission(Permissions::Administrator);
        role.add_permission(Permissions::ManageGuild);

        assert!(role.has_permission(Permissions::Administrator));
        assert!(role.has_permission(Permissions::ManageGuild));

        assert!(!role.has_permission(Permissions::SendMessages));
    }

    #[test]
    fn test_get_permissions() {
        let role = Role {
            id: String::from("1"),
            name: String::from("Moderator"),
            guild_id: String::from("guild_id1"),
            color: String::from("#00FF00"),
            position: 2,
            permissions: String::from("8208"), // MANAGE_CHANNELS + MANAGE_ROLES
            hoist: true,
            mentionable: false,
        };

        let permissions = role.get_permissions();

        assert!(permissions.contains(&Permissions::ManageChannels));
        assert!(permissions.contains(&Permissions::ManageRoles));
        assert!(!permissions.contains(&Permissions::Administrator));
    }
}
