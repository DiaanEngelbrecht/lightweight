use crate::repositories::exercises::models::Exercise;
use async_trait::async_trait;
use flair_core::store::snare::DBConnection;
use sqlx::Database;

#[async_trait]
pub trait ExerciseRepositoryContract<DB: Database> {
    async fn create_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise: Exercise,
    ) -> Result<i64, E>;
}
