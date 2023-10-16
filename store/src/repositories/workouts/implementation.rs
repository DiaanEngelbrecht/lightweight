use super::contract::WorkoutsRepositoryContract;
use super::models::Workout;
use async_trait::async_trait;
use flair_core::store::snare::{DBConnection, Ensnared};
pub struct WorkoutsRepository {}
#[async_trait]
impl WorkoutsRepositoryContract<sqlx::MySql> for WorkoutsRepository {
    async fn get_workout<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        id: i64,
    ) -> Result<Option<Workout>, E> {
        let workouts = sqlx::query_as::<_, Workout>("SELECT * FROM workouts WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await?;
        Ok(workouts)
    }
}
