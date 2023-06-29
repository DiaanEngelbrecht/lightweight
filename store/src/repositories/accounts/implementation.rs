use async_trait::async_trait;
use sqlx::Executor;

use super::contract::AccountsRepositoryContract;
use super::models::Account;

struct AccountsRepository {}

#[async_trait]
impl AccountsRepositoryContract<sqlx::MySql> for AccountsRepository {
    async fn get_accounts<'c, E>(&self, conn: E) -> Result<Vec<Account>, String>
    where
        E: Executor<'c, Database = sqlx::MySql>,
    {
        match sqlx::query_as::<_, Account>("SELECT * FROM accounts").bind(73)
            .fetch_all(conn)
            .await
        {
            Ok(articles) => Ok(articles),
            Err(_) => Err("Could not fetch accounts".to_string()),
        }
    }
}
