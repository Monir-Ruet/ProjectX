use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}
