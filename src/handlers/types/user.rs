use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct UserSearchResponse {
    id: uuid::Uuid,
    username: String,
    name: String,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub p: Option<u16>,
}
