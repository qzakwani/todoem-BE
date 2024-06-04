use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

// use crate::config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    iat: u64,
}

pub async fn jwt_auth(
    State(config): State<Config>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("{:?}", req);
    let token = match jar.get("access_token") {
        Some(token) => token.value(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    println!("{:?}", token.claims);
    return Ok(next.run(req).await);
}
