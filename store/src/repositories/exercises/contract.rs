use crate::repositories::exercises::models::Exercise;
use async_trait::async_trait;
use cali_core::store::snare::DBConnection;
use sqlx::Database;

use super::models::ExerciseCategory;

#[async_trait]
pub trait ExerciseRepositoryContract<DB: Database> {
    // Category methods
    async fn list_categories<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: Option<i64>,
    ) -> Result<Vec<ExerciseCategory>, E>;

    async fn create_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category: ExerciseCategory,
    ) -> Result<i64, E>;

    async fn update_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        name: String,
        user_id: i64,
    ) -> Result<(), E>;

    async fn delete_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        user_id: i64,
    ) -> Result<(), E>;

    // Exercise methods
    async fn list_exercises<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        user_id: Option<i64>,
    ) -> Result<Vec<Exercise>, E>;

    async fn create_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise: Exercise,
    ) -> Result<i64, E>;

    async fn update_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise_id: i64,
        name: String,
        user_id: i64,
    ) -> Result<(), E>;

    async fn delete_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise_id: i64,
        user_id: i64,
    ) -> Result<(), E>;
}
