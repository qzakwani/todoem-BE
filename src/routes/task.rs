use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::task as H;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/", post(H::create_task))
        .route("/:id", get(H::get_task))
        .route("/:id", put(H::update_task))
        .route("/:id", delete(H::delete_task))
        .route("/done/:id", put(H::done_task))
        .route("/undone/:id", put(H::undone_task))
        .route("/all", get(H::get_all_tasks))
        .route("/all/done", get(H::get_all_done_tasks))
        .route("/all/undone", get(H::get_all_undone_tasks))
        .route("/all", delete(H::delete_all_tasks))
        .route("/all/done", delete(H::delete_all_done_tasks))
        .route("/all/undone", delete(H::delete_all_undone_tasks))
}
