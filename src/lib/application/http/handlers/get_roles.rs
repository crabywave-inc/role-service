use crate::application::http::auth::UserPayload;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::member::ports::MemberService;
use crate::domain::role::entities::model::Role;
use crate::domain::role::ports::permission::PermissionService;
use crate::domain::role::ports::role::RoleService;
use axum::extract::Path;
use axum::Extension;
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn get_roles<R: RoleService, M: MemberService, P: PermissionService>(
    Extension(role_service): Extension<Arc<R>>,
    Extension(member_service): Extension<Arc<M>>,
    Extension(user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<Vec<Role>>, ApiError> {
    member_service
        .find_by_user_id(&user.id, &guild_id)
        .await
        .map_err(|_| ApiError::Forbidden("not authorized to view this ressource".to_string()))?;

    role_service
        .find_by_guild_id(&guild_id)
        .await
        .map_err(ApiError::from)
        .map(|roles| ApiSuccess::new(StatusCode::OK, roles))
}
