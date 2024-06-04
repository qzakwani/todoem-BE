// use crate::handlers::types::task as TaskTypes;
// use crate::models::AuthUser;
// use axum::extract::{Extension, Json};
use axum::http::StatusCode;

use super::types::APIError;

pub async fn create_task() -> Result<StatusCode, APIError> {
    return APIError::new::<StatusCode>(StatusCode::NOT_FOUND, format!("DIE: {:?}", "hello"));
}
