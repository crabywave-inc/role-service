use crate::application::http::auth::UserPayload;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::member::ports::MemberService;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::RoleService;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateRoleResponseData(Role);

pub async fn create_role<R: RoleService, M: MemberService>(
    Extension(role_service): Extension<Arc<R>>,
    Extension(member_service): Extension<Arc<M>>,
    Extension(user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<ApiSuccess<CreateRoleResponseData>, ApiError> {
    println!("User: {:?}", user);
    role_service
        .create_role(&guild_id, payload)
        .await
        .map_err(ApiError::from)
        .map(|role| ApiSuccess::new(StatusCode::CREATED, CreateRoleResponseData(role)))
}
