use crate::repositories::Repository;
use async_trait::async_trait;
use domain::entities::users::challenge::Challenge;
use domain::entities::users::passkey::Passkey;
use domain::entities::users::user::User;
use sqlx::Error;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_id(&self, id: Uuid) -> Result<User, Error>;
    async fn add_user(&self, user: User) -> Result<(), Error>;
    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), Error>;
    async fn update_user(&self, user: User) -> Result<(), Error>;
    async fn find_user_by_email(&self, email: String) -> Result<User, Error>;
    async fn add_update_webauthn_challenge(&self, challenge: Challenge) -> Result<(), Error>;
    async fn find_challenge(&self, user_id: Uuid) -> Result<Challenge, Error>;
    async fn add_passkey(&self, passkey: Passkey) -> Result<(), Error>;
    async fn find_passkey(&self, credential_id: String) -> Result<Passkey, Error>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_user_by_id(&self, id: Uuid) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT
                    u.*,
                    r.name AS role
                FROM users AS u
                INNER JOIN user_roles AS ur
                    ON u.id = ur.user_id
                INNER JOIN roles AS r
                    ON r.id = ur.role_id
                WHERE u.id = $1;
                "#,
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn add_user(&self, user: User) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let user_id: Uuid = sqlx::query_scalar(
            r#"
                INSERT INTO users (name, email, password, image, email_verified)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id
                "#,
        )
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.image)
            .bind(user.email_verified)
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query(
            r#"
                INSERT INTO user_roles (user_id, role_id)
                SELECT $1, id
                FROM roles
                WHERE name = $2
                "#,
        )
            .bind(user_id)
            .bind("user")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query(
            r#"
                DELETE FROM users 
                WHERE 
                    id = $1
                "#,
        )
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user(&self, user: User) -> Result<(), Error> {
        sqlx::query(
            r#"
                UPDATE users
                SET
                    name = COALESCE($2, name),
                    password = COALESCE($3, password),
                    email_verified = COALESCE($4, email_verified),
                    image = COALESCE($5, image),
                    phone = COALESCE($6, phone),
                    phone_verified = COALESCE($7, phone_verified),
                    is_active = COALESCE($8, is_active),
                    two_factor = COALESCE($9, two_factor),
                    concurrency_stamp = COALESCE($10, concurrency_stamp),
                    failed_login_count = COALESCE($11, failed_login_count),
                    lockout_end = COALESCE($12, lockout_end),
                    updated_at = NOW()
                WHERE id = $1;
                "#,
        )
            .bind(user.id)
            .bind(user.name)
            .bind(user.password)
            .bind(user.email_verified)
            .bind(user.image)
            .bind(user.phone)
            .bind(user.phone_verified)
            .bind(user.is_active)
            .bind(user.two_factor)
            .bind(user.concurrency_stamp)
            .bind(user.failed_login_count)
            .bind(user.lockout_end)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_user_by_email(&self, email: String) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            r#"
                 SELECT
                    u.*,
                    r.name AS role
                    FROM users AS u
                    INNER JOIN user_roles AS ur
                        ON u.id = ur.user_id
                    INNER JOIN roles AS r
                        ON r.id = ur.role_id
                    WHERE u.email = $1;
                 "#,
        )
            .bind(email)
            .fetch_one(&self.pool)
            .await
    }

    async fn add_update_webauthn_challenge(&self, challenge: Challenge) -> Result<(), Error> {
        sqlx::query(
            r#"
                INSERT INTO challenges (user_id, challenge, expires_at)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id)
                DO UPDATE SET
                    challenge = EXCLUDED.challenge,
                    expires_at = EXCLUDED.expires_at;
                "#,
        )
            .bind(challenge.user_id)
            .bind(challenge.challenge)
            .bind(challenge.expires_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_challenge(&self, user_id: Uuid) -> Result<Challenge, Error> {
        sqlx::query_as::<_, Challenge>(
            r#"
                SELECT * FROM challenges
                WHERE user_id = $1
                
                "#,
        )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn add_passkey(&self, passkey: Passkey) -> Result<(), Error> {
        sqlx::query(
            r#"
                INSERT INTO passkeys (
                    credential_id,
                    public_key,
                    counter,
                    device_type,
                    backed_up,
                    transports,
                    user_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                
                ON CONFLICT (credential_id)
                DO UPDATE SET
                    public_key = EXCLUDED.public_key,
                    counter = EXCLUDED.counter,
                    device_type = EXCLUDED.device_type,
                    backed_up = EXCLUDED.backed_up,
                    transports = EXCLUDED.transports,
                    user_id = EXCLUDED.user_id;
                "#,
        )
            .bind(passkey.credential_id)
            .bind(passkey.public_key)
            .bind(passkey.counter)
            .bind(passkey.device_type)
            .bind(passkey.backed_up)
            .bind(passkey.transports)
            .bind(passkey.user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_passkey(&self, credential_id: String) -> Result<Passkey, Error> {
        sqlx::query_as::<_, Passkey>(
            r#"
                SELECT * FROM passkeys
                WHERE credential_id = $1
                "#,
        )
            .bind(credential_id)
            .fetch_one(&self.pool)
            .await
    }
}
