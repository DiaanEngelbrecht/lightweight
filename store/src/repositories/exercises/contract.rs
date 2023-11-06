use crate::repositories::exercises::models::Exercise;
use async_trait::async_trait;
use cali_core::store::snare::DBConnection;
use sqlx::Database;

use super::models::ExerciseCategory;

#[async_trait]
pub trait ExerciseRepositoryContract<DB: Database> {
    async fn list_categories<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
    ) -> Result<Vec<ExerciseCategory>, E>;

    async fn create_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise: Exercise,
    ) -> Result<i64, E>;
}
