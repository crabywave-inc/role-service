use std::sync::Arc;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::RoleService;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateRoleResponseData(Role);

pub async fn create_role<R: RoleService>(
    Extension(role_service): Extension<Arc<R>>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateRoleRequest>,

) -> Result<ApiSuccess<CreateRoleResponseData>, ApiError> {
    role_service
        .create_role(&guild_id, payload)
        .await
        .map_err(ApiError::from)
        .map(|role| ApiSuccess::new(StatusCode::CREATED, CreateRoleResponseData(role)))
}
