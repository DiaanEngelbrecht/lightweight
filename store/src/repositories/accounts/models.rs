use flair_derive::Ensnare;
use sqlx::{query, FromRow};

#[derive(Clone, Debug, FromRow, Ensnare)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub salt: Vec<u8>,
    pub email: String,
    pub password: String,
}
