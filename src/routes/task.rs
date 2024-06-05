use axum::{routing::post, Router};

use crate::handlers::task as TaskHandlers;

pub fn init() -> Router {
    Router::new().route("", post(TaskHandlers::create_task))
}
