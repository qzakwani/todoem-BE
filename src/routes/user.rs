use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::user as H;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route("/:id/profile", get(H::view_user_profile))
        //* REQUEST *//
        .route("/search", get(H::search))
        .route("/:id/request", post(H::request_connection))
        .route("/:id/request", delete(H::delete_request_connection))
        .route("/:id/accept", put(H::accept_connection))
        .route("/:id/reject", put(H::reject_connection))
        .route("/requests/received", get(H::get_received_requests))
        .route("/requests/sent", get(H::get_sent_requests))
        //* LISTERS *//
        .route("/listers/page/:p", get(H::get_listers))
        .route("/listers/:id", get(H::view_lister_profile))
        .route("/listers/:id/disconnect", put(H::disconnect_lister))
}
