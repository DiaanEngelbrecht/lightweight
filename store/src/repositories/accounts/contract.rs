use crate::repositories::accounts::models::Account;
use async_trait::async_trait;
use cali_core::store::snare::DBConnection;
use sqlx::Database;

#[async_trait]
pub trait AccountsRepositoryContract<DB: Database> {
    async fn get_account<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        email: String,
    ) -> Result<Option<Account>, E>;

    async fn get_accounts<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
    ) -> Result<Vec<Account>, E>;

    async fn create_account<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        account: Account,
    ) -> Result<i64, E>;
}
