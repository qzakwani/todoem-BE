use axum::{routing::get, Router};

use crate::handlers::task as TaskHandlers;

pub fn init() -> Router {
    Router::new().route("/", get(TaskHandlers::create_task))
}
