mod config;
mod db;
mod handlers;
mod middlewares;
mod models;
mod routes;

use tracing_subscriber;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // initialize our configuration
    let config = config::init().await.unwrap();

    let router = routes::init();

    // run our app with hyper
    let addr = format!("0.0.0.0:{}", config.port);

    println!("\n\nListening on http://{}\n", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
