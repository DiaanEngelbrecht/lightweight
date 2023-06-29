use async_trait::async_trait;
use sqlx::{Database, Executor};
use crate::repositories::accounts::models::Account;


#[async_trait]
pub trait AccountsRepositoryContract<DB: Database> {
    async fn get_accounts<'c, E>(&self, conn: E) -> Result<Vec<Account>, String>
    where
        E: Executor<'c, Database = DB>;
}
