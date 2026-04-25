use sqlx::PgPool;

use crate::repositories::user::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: UserRepository,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool),
        }
    }
}
