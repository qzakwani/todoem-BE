use serde::{Deserialize, Serialize};
#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub username: String,
}
