use cali_derive::Ensnare;
use chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Exercise {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i64>,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct ExerciseCategory {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub created_by: Option<i64>,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}

impl Exercise {
    pub fn new(name: String, category_id: i64, user_id: Option<i64>) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            name,
            category_id,
            created_at: now,
            created_by: user_id,
            updated_at: now,
            updated_by: user_id,
            deleted_at: None,
            deleted_by: None,
        }
    }
}

impl ExerciseCategory {
    pub fn new(name: String, user_id: Option<i64>) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            name,
            created_at: now,
            created_by: user_id,
            updated_at: now,
            updated_by: user_id,
            deleted_at: None,
            deleted_by: None,
        }
    }
}
