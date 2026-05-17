use crate::repositories::Repository;
use async_trait::async_trait;
use domain::entities::users::session::Session;
use uuid::Uuid;

#[async_trait]
pub trait SessionRepository {
    async fn find_session_by_id(&self, id: Uuid) -> Result<Session, sqlx::Error>;
    async fn create_session(&self, session: Session) -> Result<Session, sqlx::Error>;
    async fn delete_session_by_id(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn update_session(&self, id: Uuid, session: Session) -> Result<(), sqlx::Error>;
    async fn delete_user_sessions(&self, user_id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl SessionRepository for Repository {
    async fn find_session_by_id(&self, id: Uuid) -> Result<Session, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions 
            WHERE 
                id = $1
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn create_session(&self, session: Session) -> Result<Session, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            r#"
                INSERT INTO sessions (
                    ip,
                    user_agent,
                    user_id,
                    expires_at
                )
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(session.ip)
        .bind(session.user_agent)
        .bind(session.user_id)
        .bind(session.expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete_user_sessions(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM sessions 
                WHERE 
                    user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_session_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM sessions 
                WHERE 
                    id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_session(&self, id: Uuid, session: Session) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                UPDATE sessions 
                SET 
                    ip = $1, 
                    user_agent = $2, 
                    expires_at = $3 
                WHERE 
                    id = $4
            "#,
        )
        .bind(session.ip)
        .bind(session.user_agent)
        .bind(session.expires_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
