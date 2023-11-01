use cali_derive::Ensnare;
use sqlx::FromRow;
#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Workout {
    pub id: i64,
}
