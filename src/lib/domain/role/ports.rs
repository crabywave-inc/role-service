use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use std::future::Future;

pub trait RoleService: Clone + Send + Sync + 'static {
    fn create_role(&self, server_id: &str, payload: CreateRoleRequest) -> impl Future<Output=Result<Role, RoleError>> + Send;
}

pub trait RoleRepository: Clone + Send + Sync + 'static {
    fn create(&self, server_id: &str, payload: CreateRoleRequest) -> impl Future<Output = Result<Role, RoleError>> + Send;
    fn find_by_server_id(
        &self,
        role_id: &str,
    ) -> impl Future<Output = Result<Vec<Role>, RoleError>> + Send;
}
