use crate::repositories::Repository;
use async_trait::async_trait;
use domain::entities::users::provider::Provider;

#[async_trait]
pub trait ProviderRepository {
    async fn find_provider_by_account_id(
        &self,
        account_id: String,
    ) -> Result<Provider, sqlx::Error>;
    async fn create_provider(&self, provider: Provider) -> Result<Provider, sqlx::Error>;
}

#[async_trait]
impl ProviderRepository for Repository {
    async fn find_provider_by_account_id(
        &self,
        account_id: String,
    ) -> Result<Provider, sqlx::Error> {
        let provider =
            sqlx::query_as::<_, Provider>("SELECT * FROM provider WHERE account_id = $1")
                .bind(account_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(provider)
    }

    async fn create_provider(&self, provider: Provider) -> Result<Provider, sqlx::Error> {
        let provider = sqlx::query_as::<_, Provider>(
            r#"
                INSERT INTO provider (name, account_id, user_id)
                VALUES ($1, $2, $3)
                RETURNING *
                "#,
        )
            .bind(provider.name)
            .bind(provider.account_id)
            .bind(provider.user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(provider)
    }
}
