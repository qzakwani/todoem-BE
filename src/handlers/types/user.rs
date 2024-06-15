use crate::models::user as M;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub p: Option<u16>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ViewUser {
    id: uuid::Uuid,
    username: String,
    name: Option<String>,
    pub connected: bool,
    pub sent_connection: bool,
    pub received_connection: bool,
}

impl ViewUser {
    pub fn from(user: M::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            connected: false,
            sent_connection: false,
            received_connection: false,
        }
    }
}
