use crate::repositories::workouts::models::Workout;
use async_trait::async_trait;
use cali_core::store::snare::DBConnection;
use sqlx::Database;
#[async_trait]
pub trait WorkoutsRepositoryContract<DB: Database> {
    async fn get_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<Workout>, E>;
}
