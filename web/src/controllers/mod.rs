use thiserror::Error as ThisError;
use tonic::Status;

pub mod accounts;
pub mod exercises;

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        Self::internal(format!("Error occurred: {}", err))
    }
}

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
}
