use crate::repositories::Repository;
use async_trait::async_trait;
use domain::entities::users::user::User;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_id(&self, id: Uuid) -> Result<User, sqlx::Error>;
    async fn create_user(&self, user: User) -> Result<(), sqlx::Error>;
    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn update_user(&self, id: Uuid, user: User) -> Result<(), sqlx::Error>;
    async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserRepository for Repository {
    async fn find_user_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await
    }

    async fn create_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, $4)")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(user.role)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user(&self, id: Uuid, user: User) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
