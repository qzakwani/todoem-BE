pub mod users;

use axum::Router;

pub fn init() -> Router {
    let api = Router::new().nest("/users", users::init());

    Router::new().nest("/api", api)
}
