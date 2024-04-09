use cali_derive::Ensnare;
use chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
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

impl Exercise {
    pub fn new(name: String, category_id: i64) -> Self {
        Self {
            id: 0,
            name,
            category_id,
            updated_at: Utc::now().naive_utc(),
            updated_by: None,
            deleted_at: None,
            deleted_by: None,
        }
    }
}

impl ExerciseCategory {
    pub fn new(name: String) -> Self {
        Self {
            id: 0,
            name,
            updated_at: Utc::now().naive_utc(),
            updated_by: None,
            deleted_at: None,
            deleted_by: None,
        }
    }
}
