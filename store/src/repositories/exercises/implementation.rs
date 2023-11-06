use async_trait::async_trait;
use cali_core::store::snare::{DBConnection, Ensnared};

use super::contract::ExerciseRepositoryContract;
use super::models::{Exercise, ExerciseCategory};

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

    async fn list_categories<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
    ) -> Result<Vec<ExerciseCategory>, E> {
        let exercise_categories = sqlx::query_as::<_, ExerciseCategory>("SELECT * FROM exercise_categories")
            .fetch_all(conn)
            .await?;
        Ok(exercise_categories)
    }
}
