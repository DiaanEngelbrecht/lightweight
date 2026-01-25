use cali_derive::Ensnare;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SetType {
    #[default]
    Weight,
    Timed,
    Distance,
}

impl TryFrom<String> for SetType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "weight" => Ok(SetType::Weight),
            "timed" => Ok(SetType::Timed),
            "distance" => Ok(SetType::Distance),
            _ => Err(format!("Invalid SetType: {}", value)),
        }
    }
}

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Workout {
    pub id: i64,
    pub workout_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}

impl Workout {
    pub fn new(workout_date: NaiveDate, notes: Option<String>, user_id: i64) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            workout_date,
            notes,
            created_at: now,
            created_by: user_id,
            updated_at: now,
            updated_by: Some(user_id),
            deleted_at: None,
            deleted_by: None,
        }
    }
}

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct WorkoutExercise {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub position: i32,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}

impl WorkoutExercise {
    pub fn new(
        workout_id: i64,
        exercise_id: i64,
        position: i32,
        notes: Option<String>,
        user_id: i64,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            workout_id,
            exercise_id,
            position,
            notes,
            created_at: now,
            created_by: user_id,
            updated_at: now,
            updated_by: Some(user_id),
            deleted_at: None,
            deleted_by: None,
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct ExerciseSet {
    pub id: i64,
    pub workout_exercise_id: i64,
    pub position: i32,
    #[sqlx(try_from = "String")]
    pub set_type: SetType,
    pub reps: Option<i32>,
    pub weight: Option<Decimal>,
    pub duration_seconds: Option<i32>,
    pub distance: Option<Decimal>,
    pub speed: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<i64>,
}

impl ExerciseSet {
    pub fn new_weight_set(
        workout_exercise_id: i64,
        position: i32,
        reps: Option<i32>,
        weight: Option<Decimal>,
        notes: Option<String>,
        user_id: i64,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: 0,
            workout_exercise_id,
            position,
            set_type: SetType::Weight,
            reps,
            weight,
            duration_seconds: None,
            distance: None,
            speed: None,
            notes,
            created_at: now,
            created_by: user_id,
            updated_at: now,
            updated_by: Some(user_id),
            deleted_at: None,
            deleted_by: None,
        }
    }
}

// Query result for workout exercise with exercise details
#[derive(Clone, Debug, FromRow)]
pub struct WorkoutExerciseWithDetails {
    pub id: i64,
    pub workout_id: i64,
    pub exercise_id: i64,
    pub position: i32,
    pub notes: Option<String>,
    pub exercise_name: String,
    pub category_name: String,
}

// Summary for list view
#[derive(Clone, Debug, FromRow)]
pub struct WorkoutSummary {
    pub id: i64,
    pub workout_date: NaiveDate,
    pub notes: Option<String>,
    pub exercise_count: i64,
    pub total_sets: i64,
}
