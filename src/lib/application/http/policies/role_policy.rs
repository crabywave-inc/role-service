use std::sync::Arc;

use crate::domain::role::{
    entities::{error::RoleError, permissions::Permissions},
    ports::permission::PermissionService,
};

pub struct RolePolicy;

impl RolePolicy {
    pub async fn create<P: PermissionService>(
        user_id: &str,
        guild_id: &str,
        permission_service: Arc<P>,
    ) -> Result<(), anyhow::Error> {
        let permissions = permission_service
            .get_permissions(user_id, guild_id)
            .await
            .map_err(|_| RoleError::Forbidden)?;

        let has_permission =
            Permissions::has_one_of_permissions(&permissions, &[Permissions::ManageRoles, Permissions::ManageGuild, Permissions::Administrator]);

        if !has_permission {
          return Err(RoleError::Forbidden.into());
        }
     
        Ok(())
    }
}
