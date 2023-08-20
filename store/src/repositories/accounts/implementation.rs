use async_trait::async_trait;
use flair_core::store::snare::{DBConnection, Ensnared};

use super::contract::AccountsRepositoryContract;
use super::models::Account;

pub struct AccountsRepository {}

#[async_trait]
impl AccountsRepositoryContract<sqlx::MySql> for AccountsRepository {
    async fn get_accounts<'c, C: DBConnection<'c>>(conn: C) -> Result<Vec<Account>, String> {
        match sqlx::query_as::<_, Account>("SELECT * FROM accounts")
            .fetch_all(conn)
            .await
        {
            Ok(articles) => Ok(articles),
            Err(_) => Err("Could not fetch accounts".to_string()),
        }
    }

    async fn get_account<'c, C: DBConnection<'c>>(
        conn: C,
        email: String,
    ) -> Result<Option<Account>, String> {
        match sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE email = ?")
            .bind(email)
            .fetch_optional(conn)
            .await
        {
            Ok(articles) => Ok(articles),
            Err(_) => Err("Could not fetch accounts".to_string()),
        }
    }

    async fn create_account<'c, C: DBConnection<'c>>(
        conn: C,
        account: Account,
    ) -> Result<i64, sqlx::Error> {
        let result = account.trap("accounts").insert().execute(conn).await;

        result.map(|r| r.last_insert_id() as i64)
    }
}
