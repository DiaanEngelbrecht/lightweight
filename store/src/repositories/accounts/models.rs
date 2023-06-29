use flair_derive::Ensnare;
use sqlx::{query, FromRow};

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Account {
    id: i64,
    name: String,
    email: String,
    password: String,
}
