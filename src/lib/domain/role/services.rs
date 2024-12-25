use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::{RoleRepository, RoleService};

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

impl<R> RoleService for RoleServiceImpl<R> where R: RoleRepository {
    async fn create_role(&self, server_id: &str, payload: CreateRoleRequest) -> Result<Role, RoleError> {
        self.role_repository.create(server_id, payload).await
    }
}
