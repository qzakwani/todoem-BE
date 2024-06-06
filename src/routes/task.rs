use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::handlers::task as TaskHandlers;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new().route("/", post(TaskHandlers::create_task))
}
