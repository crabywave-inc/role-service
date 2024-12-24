use axum::Extension;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::role::ports::RoleService;

async fn get_roles<R>(
    Extension(role_service): Extension<R>
) -> Result<ApiSuccess<String>, ApiError>
where
    R: RoleService
{
    todo!("Implement the get_roles handler")
}