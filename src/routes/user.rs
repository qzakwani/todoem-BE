use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::user as H;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/search", get(H::search))
        .route("/:id/profile", get(H::view_user_profile))
        .route("/:id/request", post(H::request_connection))
        .route("/:id/request", delete(H::delete_request_connection))
        .route("/:id/accept", put(H::accept_connection))
        .route("/:id/reject", put(H::reject_connection))
}
