use axum::{
    // routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::user as H;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new().route("/search", H::search)
}
