use thiserror::Error as ThisError;
use tonic::Status;

pub mod accounts;
pub mod exercises;

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DatabaseError(msg) => Self::internal(format!("Database error: {}", msg)),
            AppError::Unauthorized(msg) => Self::unauthenticated(msg),
            AppError::Forbidden(msg) => Self::permission_denied(msg),
        }
    }
}

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("database error - {0}")]
    DatabaseError(String),

    #[error("unauthorized - {0}")]
    Unauthorized(String),

    #[error("forbidden - {0}")]
    Forbidden(String),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        let msg: String = match value {
            sqlx::Error::Database(d) => d.message().to_string(),
            sqlx::Error::RowNotFound => "row not found".to_string(),
            sqlx::Error::ColumnNotFound(_) => "column not found".to_string(),
            _ => "generic database catch all error".to_string(),
        };

        Self::DatabaseError(msg)
    }
}
