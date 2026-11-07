use deadpool_redis::Pool;
use crate::repositories::Repository;

pub mod session;
pub mod user;
pub mod passkey;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Services {
    repo: Repository,
    redis: Pool
}

impl Services {
    pub fn new(repo: Repository, redis: Pool) -> Self {
        Self { repo, redis }
    }
}
