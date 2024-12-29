use crate::application::http::auth::UserPayload;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::member::ports::MemberService;
use crate::domain::role::ports::permission::PermissionService;
use crate::domain::role::ports::role::RoleService;
use axum::extract::Path;
use axum::Extension;
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn get_roles<R: RoleService, M: MemberService, P: PermissionService>(
    Extension(_role_service): Extension<Arc<R>>,
    Extension(permission_service): Extension<Arc<P>>,
    Extension(user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<String>, ApiError> {
    println!("User: {:?}", user);
    let permissions = permission_service.get_permissions(&user.id, &guild_id).await.map_err(ApiError::from)?;

    for p in permissions {
        println!("Permission: {:?}", p);
    }
 
    Ok(ApiSuccess::new(StatusCode::OK, "Roles".to_string()))
}
