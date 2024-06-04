pub mod group;
pub mod list;
pub mod task;
pub mod user;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub email: String,
}
