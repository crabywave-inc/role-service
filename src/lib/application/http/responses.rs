use crate::domain::role::entities::error::{PermissionError, RoleError};
use crate::{application::http::handlers::ApiError, domain::member::entities::error::MemberError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiErrorDetail>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub rule: String,
    pub field: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiResponseError {
    pub code: String,
    pub status: u16,
    pub message: String,
}

impl From<RoleError> for ApiError {
    fn from(value: RoleError) -> Self {
        match value {
            RoleError::InternalServerError => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
            RoleError::NotFound => ApiError::NotFound("Guild not found".to_string()),
            RoleError::CreateError(e) => ApiError::UnProcessableEntity(e),
            RoleError::Unauthorized => ApiError::Unauthorized("Unauthorized".to_string()),
        }
    }
}

impl From<MemberError> for ApiError {
    fn from(value: MemberError) -> Self {
        match value {
            MemberError::NotFound(e) => ApiError::NotFound(e),
            MemberError::CreateError(e) => ApiError::UnProcessableEntity(e),
            MemberError::InternalServerError => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
            MemberError::Unauthorized => ApiError::Unauthorized("Unauthorized".to_string()),
            MemberError::UpdateError(e) => ApiError::UnProcessableEntity(e),
        }
    }
}

impl From<PermissionError> for ApiError {
    fn from(value: PermissionError) -> Self {
        match value {
            PermissionError::NotFound => ApiError::NotFound("Permission not found".to_string()),
            PermissionError::MemberError(e) => ApiError::InternalServerError(e),
            PermissionError::RoleError(e) => ApiError::InternalServerError(e),
        }
    }
}
