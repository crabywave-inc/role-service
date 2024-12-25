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

impl Permissions {
    pub fn from_bitfield(bitfield: u64) -> Vec<Self> {
        let all_permissions = [
            Permissions::Administrator,
            Permissions::ManageChannels,
            Permissions::ManageGuild,
            Permissions::ViewChannel,
            Permissions::SendMessages,
            Permissions::ManageMessages,
            Permissions::ManageRoles,
        ];

        all_permissions
            .iter()
            .copied()
            .filter(|&permission| (bitfield & (permission as u64)) == (permission as u64))
            .collect()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Permissions::Administrator => "Administrator",
            Permissions::ManageChannels => "Manage Channels",
            Permissions::ManageGuild => "Manage Guild",
            Permissions::ViewChannel => "View Channel",
            Permissions::SendMessages => "Send Messages",
            Permissions::ManageMessages => "Manage Messages",
            Permissions::ManageRoles => "Manage Roles",
        }
    }
}