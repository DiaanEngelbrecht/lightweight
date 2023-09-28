use async_trait::async_trait;
use flair_core::store::snare::{DBConnection, Ensnared};

use super::contract::AccountsRepositoryContract;
use super::models::Account;

pub struct AccountsRepository {}

#[async_trait]
impl AccountsRepositoryContract<sqlx::MySql> for AccountsRepository {
    async fn get_accounts<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
    ) -> Result<Vec<Account>, E> {
        let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
            .fetch_all(conn)
            .await?;
        Ok(accounts)
    }

    async fn get_account<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        email: String,
    ) -> Result<Option<Account>, E> {
        let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE email = ?")
            .bind(email)
            .fetch_optional(conn)
            .await?;
        Ok(accounts)
    }

    async fn create_account<'c, C: DBConnection<'c>, E: From<sqlx::Error>>(
        conn: C,
        account: Account,
    ) -> Result<i64, E> {
        let result = account.trap("accounts").insert().execute(conn).await?;

        Ok(result.last_insert_id() as i64)
    }
}
