use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum MemberError {
    #[error("Member not found: {0}")]
    NotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to create member: {0}")]
    CreateError(String),
    #[error("Failed to update member: {0}")]
    UpdateError(String),
}
