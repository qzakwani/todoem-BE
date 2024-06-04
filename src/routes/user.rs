use axum::{routing::get, Router};

use crate::handlers::user;

pub fn init() -> Router {
    Router::new().route("/", get(user::get_user))
}
