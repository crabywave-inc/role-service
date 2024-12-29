use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RoleError {
    #[error("Guild not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to create guild: {0}")]
    CreateError(String),
}

#[derive(Debug, Clone, Error)]
pub enum PermissionError {
    #[error("Permission not found")]
    NotFound,
    #[error("MemberError: {0}")]
    MemberError(String),
    #[error("RoleError: {0}")]
    RoleError(String),
}
