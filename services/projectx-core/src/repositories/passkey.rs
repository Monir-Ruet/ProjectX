use crate::repositories::Repository;
use async_trait::async_trait;
use domain::entities::users::passkey::Passkey;
use sqlx::Error;

#[async_trait]
pub trait PasskeyRepository {
    async fn add_passkey(&self, passkey: Passkey) -> Result<(), Error>;
    async fn find_passkey(&self, cred_id: Vec<u8>) -> Result<Passkey, Error>;
    async fn find_all_passkeys(&self, email: String) -> Result<Vec<Passkey>, Error>;
    async fn update_passkey(&self, updated_passkey: Passkey) -> Result<(), Error>;
}

#[async_trait]
impl PasskeyRepository for Repository {
    async fn add_passkey(&self, passkey: Passkey) -> Result<(), Error> {
        sqlx::query(
            r#"
                INSERT INTO passkeys (user_id, cred_id, device_type, passkey)
                VALUES ($1, $2, $3, $4)
                "#,
        )
            .bind(passkey.user_id)
            .bind(passkey.cred_id)
            .bind(passkey.device_type)
            .bind(passkey.passkey)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_passkey(&self, cred_id: Vec<u8>) -> Result<Passkey, Error> {
        sqlx::query_as::<_, Passkey>(
            r#"
                SELECT * FROM passkeys
                WHERE cred_id = $1
                "#,
        )
            .bind(cred_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_all_passkeys(&self, email: String) -> Result<Vec<Passkey>, Error> {
        sqlx::query_as::<_, Passkey>(
            r#"
                SELECT p.*
                FROM passkeys as p
                JOIN users AS u ON u.id = p.user_id
                WHERE u.email = $1
                "#,
        )
            .bind(email)
            .fetch_all(&self.pool)
            .await
    }

    async fn update_passkey(&self, updated_passkey: Passkey) -> Result<(), Error> {
        sqlx::query(
            r#"
                    UPDATE passkeys
                    SET passkey = $1
                    WHERE cred_id = $2
                "#,
        )
            .bind(updated_passkey.passkey)
            .bind(updated_passkey.cred_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}