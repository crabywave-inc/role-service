use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tracing::warn;

use crate::domain::{
    member::ports::MemberService,
    role::{
        entities::{error::PermissionError, permissions::Permissions},
        ports::{permission::PermissionService, role::RoleService},
    },
};

#[derive(Debug, Clone)]
pub struct PermissionServiceImpl<R, M>
where
    R: RoleService,
    M: MemberService,
{
    pub role_service: Arc<R>,
    pub member_service: Arc<M>,
}

impl<R, M> PermissionServiceImpl<R, M>
where
    R: RoleService,
    M: MemberService,
{
    pub fn new(role_service: Arc<R>, member_service: Arc<M>) -> Self {
        Self {
            role_service,
            member_service,
        }
    }
}

impl<R, M> PermissionService for PermissionServiceImpl<R, M>
where
    R: RoleService,
    M: MemberService,
{
    async fn get_permissions(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> Result<Vec<Permissions>, PermissionError> {
        let member = self
            .member_service
            .find_by_user_id(user_id, guild_id)
            .await
            .map_err(|e| PermissionError::MemberError(e.to_string()))?;

        let roles = self
            .member_service
            .get_roles(&member.id)
            .await
            .map_err(|e| PermissionError::RoleError(e.to_string()))?;

        let mut permissions_set = HashSet::new();

        for role in roles {
            match role.permissions.parse::<u64>() {
                Ok(permission_bitfield) => {
                    let permissions = Permissions::from_bitfield(permission_bitfield);
                    permissions_set.extend(permissions);
                }
                Err(_) => {
                    warn!(
                        "Failed to parse permissions bitfield: {} for role ID: {}",
                        role.permissions, role.id
                    );
                }
            }
        }

        let permissions = permissions_set.into_iter().collect::<Vec<_>>();

        Ok(permissions)
    }
}
