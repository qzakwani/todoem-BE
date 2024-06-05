use crate::models::AuthUser;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::{error, instrument};

// use crate::config;

#[instrument]
pub async fn jwt_auth(
    State((secret_key, jwt_validation)): State<(String, Validation)>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match jar.get("access_token") {
        Some(token) => token.value(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let jwt_key = DecodingKey::from_secret(secret_key.as_ref());

    let auth_user = match decode::<AuthUser>(&token, &jwt_key, &jwt_validation) {
        Ok(token_data) => token_data.claims,

        Err(e) => {
            error!("Error decoding token: {:#?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    req.extensions_mut().insert(auth_user);

    return Ok(next.run(req).await);
}
