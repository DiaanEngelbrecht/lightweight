use chrono::NaiveDateTime;
use flair_derive::Ensnare;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub updated_at: NaiveDateTime,
    pub updated_by: i64,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}


#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct ExerciseCategory {
    pub id: i64,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}
