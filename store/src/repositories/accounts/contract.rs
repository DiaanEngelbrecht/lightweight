use crate::repositories::accounts::models::Account;
use async_trait::async_trait;
use sqlx::{Database, Executor};

pub trait DBConnection<'c>: Executor<'c, Database = sqlx::MySql> {}

#[async_trait]
pub trait AccountsRepositoryContract<DB: Database> {
    async fn get_accounts<'c, C: DBConnection<'c>>(conn: C) -> Result<Vec<Account>, String>;

    async fn create_account<'c, C: DBConnection<'c>>(
        conn: C,
        account: Account,
    ) -> Result<i64, sqlx::Error>;
}
