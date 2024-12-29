use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::role::{RoleRepository, RoleService};

#[derive(Debug, Clone)]
pub struct RoleServiceImpl<R>
where
    R: RoleRepository,
{
    pub role_repository: R,
}

impl<R> RoleServiceImpl<R>
where
    R: RoleRepository,
{
    pub fn new(role_repository: R) -> Self {
        Self { role_repository }
    }
}

impl<R> RoleService for RoleServiceImpl<R>
where
    R: RoleRepository,
{
    async fn create_role(
        &self,
        server_id: &str,
        payload: CreateRoleRequest,
    ) -> Result<Role, RoleError> {
        self.role_repository.create(server_id, payload).await
    }

    async fn get_roles(&self, role_ids: Vec<String>) -> Result<Vec<Role>, RoleError> {
        let mut roles: Vec<Role> = Vec::new();

        for role_id in role_ids {
            let role = self.role_repository.find_by_id(&role_id).await?;
            match role {
                Some(role) => roles.push(role),
                None => return Err(RoleError::NotFound),
            }
        }

        Ok(roles)
    }
}
