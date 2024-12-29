use std::future::Future;

use crate::domain::role::entities::{error::PermissionError, permissions::Permissions};

pub trait PermissionService: Clone + Send + Sync + 'static {
    fn get_permissions(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> impl Future<Output = Result<Vec<Permissions>, PermissionError>> + Send;
}
