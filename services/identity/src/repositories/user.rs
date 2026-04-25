use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::PgPool;
use sqlx::prelude::FromRow;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: None,
            name,
            email,
            password,
        }
    }
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, StatusCode> {
        match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn create_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)")
            .bind(user.name)
            .bind(user.email)
            .bind(user.password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_user_by_email(&self, email: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_user(&self, id: String, user: User) -> Result<(), sqlx::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_user() -> Result<(), sqlx::Error> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        let repo = UserRepository::new(pool);
        let user = User {
            id: None,
            name: "Bob".into(),
            email: "bob@example.com".into(),
            password: "password".into(),
        };
        repo.create_user(user).await.expect("Failed to create user");
        Ok(())
    }

    #[tokio::test]
    async fn get_user() -> Result<(), sqlx::Error> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        let repo = UserRepository::new(pool);
        let user = repo
            .get_user_by_email("bob@example.com")
            .await
            .expect("Failed to get user");
        println!("User: {:?}", user);
        assert_eq!(user.name, "Bob");
        Ok(())
    }
}
