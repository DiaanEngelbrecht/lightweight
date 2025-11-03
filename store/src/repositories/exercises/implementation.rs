use async_trait::async_trait;
use cali_core::store::snare::{DBConnection, Ensnared};
use chrono::Utc;

use super::contract::ExerciseRepositoryContract;
use super::models::{Exercise, ExerciseCategory};

pub struct ExerciseRepository {}

#[async_trait]
impl ExerciseRepositoryContract<sqlx::MySql> for ExerciseRepository {
    // Category methods
    async fn list_categories<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: Option<i64>,
    ) -> Result<Vec<ExerciseCategory>, E> {
        let categories = if let Some(uid) = user_id {
            // Get user's own categories + system categories (created_by = 0)
            sqlx::query_as::<_, ExerciseCategory>(
                "SELECT * FROM exercise_categories
                 WHERE deleted_at IS NULL
                 AND (created_by = ? OR created_by = 0)
                 ORDER BY name",
            )
            .bind(uid)
            .fetch_all(conn)
            .await?
        } else {
            // Get only system categories
            sqlx::query_as::<_, ExerciseCategory>(
                "SELECT * FROM exercise_categories
                 WHERE deleted_at IS NULL
                 AND created_by = 0
                 ORDER BY name",
            )
            .fetch_all(conn)
            .await?
        };
        Ok(categories)
    }

    async fn create_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category: ExerciseCategory,
    ) -> Result<i64, E> {
        let result = category
            .trap("exercise_categories")
            .insert()
            .execute(conn)
            .await?;
        Ok(result.last_insert_id() as i64)
    }

    async fn update_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        name: String,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercise_categories
             SET name = ?, updated_at = ?, updated_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(name)
        .bind(now)
        .bind(user_id)
        .bind(category_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn delete_category<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercise_categories
             SET deleted_at = ?, deleted_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(user_id)
        .bind(category_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    // Exercise methods
    async fn list_exercises<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        category_id: i64,
        user_id: Option<i64>,
    ) -> Result<Vec<Exercise>, E> {
        let exercises = if let Some(uid) = user_id {
            // Get user's own exercises + system exercises (created_by = 0) for this category
            sqlx::query_as::<_, Exercise>(
                "SELECT * FROM exercises
                 WHERE category_id = ?
                 AND deleted_at IS NULL
                 AND (created_by = ? OR created_by = 0)
                 ORDER BY name",
            )
            .bind(category_id)
            .bind(uid)
            .fetch_all(conn)
            .await?
        } else {
            // Get only system exercises
            sqlx::query_as::<_, Exercise>(
                "SELECT * FROM exercises
                 WHERE category_id = ?
                 AND deleted_at IS NULL
                 AND created_by = 0
                 ORDER BY name",
            )
            .bind(category_id)
            .fetch_all(conn)
            .await?
        };
        Ok(exercises)
    }

    async fn create_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise: Exercise,
    ) -> Result<i64, E> {
        let result = exercise.trap("exercises").insert().execute(conn).await?;
        Ok(result.last_insert_id() as i64)
    }

    async fn update_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise_id: i64,
        name: String,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercises
             SET name = ?, updated_at = ?, updated_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(name)
        .bind(now)
        .bind(user_id)
        .bind(exercise_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn delete_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        exercise_id: i64,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercises
             SET deleted_at = ?, deleted_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(user_id)
        .bind(exercise_id)
        .execute(conn)
        .await?;
        Ok(())
    }
}
