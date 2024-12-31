use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use std::future::Future;

pub trait RoleService: Clone + Send + Sync + 'static {
    fn create_role(
        &self,
        guild_id: &str,
        payload: CreateRoleRequest,
    ) -> impl Future<Output = Result<Role, RoleError>> + Send;
    // récupérer les rôles à partir d'une liste de rôles ids
    fn get_roles(
        &self,
        role_ids: Vec<String>,
    ) -> impl Future<Output = Result<Vec<Role>, RoleError>> + Send;
    fn find_by_guild_id(
        &self,
        guild_id: &str,
    ) -> impl Future<Output = Result<Vec<Role>, RoleError>> + Send;
}

pub trait RoleRepository: Clone + Send + Sync + 'static {
    fn create(
        &self,
        guild_id: &str,
        payload: CreateRoleRequest,
    ) -> impl Future<Output = Result<Role, RoleError>> + Send;
    fn find_by_guild_id(
        &self,
        guild_id: &str,
    ) -> impl Future<Output = Result<Vec<Role>, RoleError>> + Send;
    fn find_by_id(
        &self,
        role_id: &str,
    ) -> impl Future<Output = Result<Option<Role>, RoleError>> + Send;
}
