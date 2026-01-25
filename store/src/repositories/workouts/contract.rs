use crate::repositories::workouts::models::{
    ExerciseSet, Workout, WorkoutExercise, WorkoutExerciseWithDetails, WorkoutSummary,
};
use async_trait::async_trait;
use cali_core::store::snare::DBConnection;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::Database;

#[async_trait]
pub trait WorkoutsRepositoryContract<DB: Database> {
    // Workout methods
    async fn create_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout: Workout,
    ) -> Result<i64, E>;

    async fn get_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<Workout>, E>;

    async fn get_workout_by_date<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: i64,
        date: NaiveDate,
    ) -> Result<Option<Workout>, E>;

    async fn list_workouts<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        user_id: i64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<WorkoutSummary>, E>;

    async fn update_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E>;

    async fn delete_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
        user_id: i64,
    ) -> Result<(), E>;

    // Workout Exercise methods
    async fn add_exercise_to_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise: WorkoutExercise,
    ) -> Result<i64, E>;

    async fn get_workout_exercises<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
    ) -> Result<Vec<WorkoutExerciseWithDetails>, E>;

    async fn get_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<WorkoutExercise>, E>;

    async fn update_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E>;

    async fn delete_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
        user_id: i64,
    ) -> Result<(), E>;

    async fn get_next_exercise_position<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_id: i64,
    ) -> Result<i32, E>;

    // Exercise Set methods
    async fn create_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set: ExerciseSet,
    ) -> Result<i64, E>;

    async fn get_sets_for_workout_exercise<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
    ) -> Result<Vec<ExerciseSet>, E>;

    async fn get_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<ExerciseSet>, E>;

    async fn update_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set_id: i64,
        reps: Option<i32>,
        weight: Option<Decimal>,
        notes: Option<String>,
        user_id: i64,
    ) -> Result<(), E>;

    async fn delete_set<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        set_id: i64,
        user_id: i64,
    ) -> Result<(), E>;

    async fn get_next_set_position<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        workout_exercise_id: i64,
    ) -> Result<i32, E>;
}
