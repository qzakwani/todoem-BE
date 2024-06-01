use axum::{routing::get, Router};

use crate::handlers::users;

pub fn init() -> Router {
    Router::new().route("/", get(users::get_user))
}
