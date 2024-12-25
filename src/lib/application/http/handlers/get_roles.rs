use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::role::ports::RoleService;
use axum::Extension;
use std::sync::Arc;

pub async fn get_roles<R>(
    Extension(_role_service): Extension<Arc<R>>,
) -> Result<ApiSuccess<String>, ApiError>
where
    R: RoleService,
{
    todo!("Implement the get_roles handler")
}
