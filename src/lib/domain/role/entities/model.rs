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

impl Role {
    pub fn new(id: &str, name: &str, server_id: &str, color: &str, position: i32) -> Self {
        Role {
            id: id.to_string(),
            name: name.to_string(),
            server_id: server_id.to_string(),
            color: color.to_string(),
            position,
            permissions: "0".to_string(),
            hoist: false,
            mentionable: false,
        }
    }

    pub fn add_permission(&mut self, permission: Permissions) {
        let current_permissions = u64::from_str_radix(&self.permissions, 16).unwrap_or(0);
        self.permissions = format!("{:x}", current_permissions | (permission as u64));
    }

    pub fn has_permission(&self, permission: Permissions) -> bool {
        let current_permissions = u64::from_str_radix(&self.permissions, 16).unwrap_or(0);
        (current_permissions & (permission as u64)) == (permission as u64)
    }
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permissions {
    Administrator = 0x0000000000000008, // 1 << 3
    ManageChannels = 0x0000000000000010, // 1 << 4
    ManageGuild = 0x0000000000000020, // 1 << 5
    ViewChannel = 0x0000000000000400, // 1 << 10
    SendMessages = 0x0000000000000800, // 1 << 11
    ManageMessages = 0x0000000000001000, // 1 << 12
    ManageRoles = 0x0000000000002000, // 1 << 13
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

}