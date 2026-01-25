use super::contract::WorkoutsRepositoryContract;
use super::models::{
    ExerciseSet, SetType, Workout, WorkoutExercise, WorkoutExerciseWithDetails, WorkoutSummary,
};
use async_trait::async_trait;
use cali_core::store::snare::{DBConnection, Ensnared};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;

pub struct WorkoutsRepository {}

#[async_trait]
impl WorkoutsRepositoryContract<sqlx::MySql> for WorkoutsRepository {
    // Workout methods
    async fn create_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout: Workout,
    ) -> Result<i64, E> {
        let result = workout.trap("workouts").insert().execute(conn).await?;
        Ok(result.last_insert_id() as i64)
    }

    async fn get_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<Workout>, E> {
        let workout = sqlx::query_as::<_, Workout>(
            "SELECT * FROM workouts WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(workout)
    }

    async fn get_workout_by_date<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Option<Workout>, E> {
        let workout = sqlx::query_as::<_, Workout>(
            "SELECT * FROM workouts
             WHERE created_by = ? AND workout_date = ? AND deleted_at IS NULL",
        )
        .bind(user_id)
        .bind(date)
        .fetch_optional(conn)
        .await?;
        Ok(workout)
    }

    async fn list_workouts<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: i64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<WorkoutSummary>, E> {
        let workouts = sqlx::query_as::<_, WorkoutSummary>(
            "SELECT
                w.id,
                w.workout_date,
                w.notes,
                COUNT(DISTINCT we.id) as exercise_count,
                COUNT(DISTINCT es.id) as total_sets
             FROM workouts w
             LEFT JOIN workout_exercises we ON we.workout_id = w.id AND we.deleted_at IS NULL
             LEFT JOIN exercise_sets es ON es.workout_exercise_id = we.id AND es.deleted_at IS NULL
             WHERE w.created_by = ? AND w.deleted_at IS NULL
             GROUP BY w.id
             ORDER BY w.workout_date DESC
             LIMIT ? OFFSET ?",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(conn)
        .await?;
        Ok(workouts)
    }

    async fn update_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workouts SET notes = ?, updated_at = ?, updated_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(notes)
        .bind(now)
        .bind(user_id)
        .bind(workout_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn delete_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workouts SET deleted_at = ?, deleted_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(user_id)
        .bind(workout_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    // Workout Exercise methods
    async fn add_exercise_to_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise: WorkoutExercise,
    ) -> Result<i64, E> {
        let result = workout_exercise
            .trap("workout_exercises")
            .insert()
            .execute(conn)
            .await?;
        Ok(result.last_insert_id() as i64)
    }

    async fn get_workout_exercises<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
    ) -> Result<Vec<WorkoutExerciseWithDetails>, E> {
        let exercises = sqlx::query_as::<_, WorkoutExerciseWithDetails>(
            "SELECT
                we.id,
                we.workout_id,
                we.exercise_id,
                we.position,
                we.notes,
                e.name as exercise_name,
                ec.name as category_name
             FROM workout_exercises we
             JOIN exercises e ON e.id = we.exercise_id
             JOIN exercise_categories ec ON ec.id = e.category_id
             WHERE we.workout_id = ? AND we.deleted_at IS NULL
             ORDER BY we.position",
        )
        .bind(workout_id)
        .fetch_all(conn)
        .await?;
        Ok(exercises)
    }

    async fn get_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<WorkoutExercise>, E> {
        let exercise = sqlx::query_as::<_, WorkoutExercise>(
            "SELECT * FROM workout_exercises WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(exercise)
    }

    async fn update_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workout_exercises SET notes = ?, updated_at = ?, updated_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(notes)
        .bind(now)
        .bind(user_id)
        .bind(workout_exercise_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn delete_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workout_exercises SET deleted_at = ?, deleted_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(user_id)
        .bind(workout_exercise_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn get_next_exercise_position<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
    ) -> Result<i32, E> {
        let result: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM workout_exercises
             WHERE workout_id = ? AND deleted_at IS NULL",
        )
        .bind(workout_id)
        .fetch_one(conn)
        .await?;
        Ok(result.0)
    }

    // Exercise Set methods
    async fn create_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set: ExerciseSet,
    ) -> Result<i64, E> {
        // Manual insert because Ensnare doesn't handle MySQL ENUMs properly
        let set_type_str = match set.set_type {
            SetType::Weight => "weight",
            SetType::Timed => "timed",
            SetType::Distance => "distance",
        };

        let result = sqlx::query(
            "INSERT INTO exercise_sets
             (workout_exercise_id, position, set_type, reps, weight, duration_seconds, distance, speed, notes, created_at, created_by, updated_at, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(set.workout_exercise_id)
        .bind(set.position)
        .bind(set_type_str)
        .bind(set.reps)
        .bind(set.weight)
        .bind(set.duration_seconds)
        .bind(set.distance)
        .bind(set.speed)
        .bind(set.notes)
        .bind(set.created_at)
        .bind(set.created_by)
        .bind(set.updated_at)
        .bind(set.updated_by)
        .execute(conn)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    async fn get_sets_for_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
    ) -> Result<Vec<ExerciseSet>, E> {
        let sets = sqlx::query_as::<_, ExerciseSet>(
            "SELECT * FROM exercise_sets
             WHERE workout_exercise_id = ? AND deleted_at IS NULL
             ORDER BY position",
        )
        .bind(workout_exercise_id)
        .fetch_all(conn)
        .await?;
        Ok(sets)
    }

    async fn get_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<ExerciseSet>, E> {
        let set = sqlx::query_as::<_, ExerciseSet>(
            "SELECT * FROM exercise_sets WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(set)
    }

    async fn update_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set_id: i64,
        reps: Option<i32>,
        weight: Option<Decimal>,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercise_sets
             SET reps = ?, weight = ?, notes = ?, updated_at = ?, updated_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(reps)
        .bind(weight)
        .bind(notes)
        .bind(now)
        .bind(user_id)
        .bind(set_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn delete_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set_id: i64,
        user_id: i64,
    ) -> Result<(), E> {
        let now = Utc::now().naive_utc();
        sqlx::query(
            "UPDATE exercise_sets SET deleted_at = ?, deleted_by = ?
             WHERE id = ? AND deleted_at IS NULL",
        )
        .bind(now)
        .bind(user_id)
        .bind(set_id)
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn get_next_set_position<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
    ) -> Result<i32, E> {
        let result: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM exercise_sets
             WHERE workout_exercise_id = ? AND deleted_at IS NULL",
        )
        .bind(workout_exercise_id)
        .fetch_one(conn)
        .await?;
        Ok(result.0)
    }
}
