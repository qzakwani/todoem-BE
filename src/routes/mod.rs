pub mod users;

use axum::{middleware, Router};
use http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::middlewares::jwt::jwt_auth;

pub fn init(config: &Config) -> Router {
    let apis = Router::new().nest("/users", users::init());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new().nest("/api", apis).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .layer(middleware::from_fn_with_state(config.clone(), jwt_auth)),
    )
}
