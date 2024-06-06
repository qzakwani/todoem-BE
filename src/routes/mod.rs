pub mod task;
pub mod user;

use crate::config::Config;
use crate::errors;
use crate::middlewares::jwt::jwt_auth;
use axum::{error_handling::HandleErrorLayer, middleware, Router};
use http::Method;
use sqlx::PgPool;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn init(config: Config) -> Router {
    let apis = Router::<PgPool>::new().nest("/task", task::init());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .nest("/api", apis)
        .with_state(config.pool)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(middleware::from_fn_with_state(
                    (config.secret_key, config.jwt_validation),
                    jwt_auth,
                ))
                .layer(HandleErrorLayer::new(errors::handle_api_error))
                .timeout(Duration::from_secs(30)),
        )
}
