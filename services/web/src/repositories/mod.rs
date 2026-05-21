use sqlx::PgPool;

pub mod provider;
pub mod session;
pub mod user;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
