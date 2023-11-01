use async_trait::async_trait;
use cali_core::store::snare::{DBConnection, Ensnared};

use super::contract::ExerciseRepositoryContract;
use super::models::Exercise;

pub struct ExerciseRepository {}

#[async_trait]
impl ExerciseRepositoryContract<sqlx::MySql> for ExerciseRepository {
    async fn create_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise: Exercise,
    ) -> Result<i64, E> {
        let result = exercise.trap("exercises").insert().execute(conn).await?;

        Ok(result.last_insert_id() as i64)
    }
}
