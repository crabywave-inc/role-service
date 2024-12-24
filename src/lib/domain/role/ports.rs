use std::future::Future;
use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::Role;

pub trait RoleService: Clone + Send + Sync + 'static {}

pub trait RoleRepository: Clone + Send + Sync + 'static {
    fn create(&self) -> impl Future<Output = Result<Role, RoleError>> + Send;
    fn find_by_server_id(&self, role_id: &str) -> impl Future<Output=Result<Vec<Role>, RoleError>> + Send;
}
