use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum MemberError {
    #[error("Member not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to create member: {0}")]
    CreateError(String),
}
