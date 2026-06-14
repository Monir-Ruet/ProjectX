use crate::repositories::Repository;

pub mod session;
pub mod user;
pub mod passkey;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Services {
    repo: Repository,
}

impl Services {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }
}
