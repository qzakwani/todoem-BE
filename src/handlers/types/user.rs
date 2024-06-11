use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct UserSearchResponse {
    id: uuid::Uuid,
    username: String,
    name: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub p: Option<u16>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ViewUserResponse {
    id: uuid::Uuid,
    username: String,
    name: Option<String>,
    pub connected: bool,
    pub sent_connection: bool,
    pub received_connection: bool,
}
