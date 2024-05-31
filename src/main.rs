mod config;

use axum::{extract, routing::post, Router};
use tracing_subscriber;

use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize our configuration
    let config = config::init().await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route(
            "/",
            post(|extract::Json(payload): extract::Json<CreateUser>| async {
                sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
                    .bind(payload.username)
                    .bind(payload.password)
                    .execute(&config.pool)
                    .await
                    .unwrap();
            }),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
