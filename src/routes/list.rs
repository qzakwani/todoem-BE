use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

use crate::handlers::list as H;

pub fn init() -> Router<PgPool> {
    Router::<PgPool>::new().route("/", post(H::create_list))
}
