use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    BoxError,
};
use tracing::instrument;

pub struct APIError(StatusCode, String);

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        #[derive(serde::Serialize)]
        struct E {
            error: String,
        }

        (self.0, Json(E { error: self.1 })).into_response()
    }
}

impl APIError {
    pub fn new(status: StatusCode, msg: &str) -> Self {
        Self(status, msg.to_string())
    }

    pub fn server() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
    }

    pub fn bad(msg: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, msg)
    }

    pub fn not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, "Not found")
    }

    pub fn auth() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "Unauthorized")
    }

    pub fn forbidden() -> Self {
        Self::new(StatusCode::FORBIDDEN, "Forbidden")
    }
}

#[instrument]
pub async fn handle_api_error(err: BoxError) -> APIError {
    tracing::error!("Error: {:#?}", err);
    if err.is::<tower::timeout::error::Elapsed>() {
        APIError::new(StatusCode::REQUEST_TIMEOUT, "Request timed out")
    } else {
        APIError::server()
    }
}
