pub mod task;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

pub struct APIError(String, StatusCode);

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        #[derive(serde::Serialize)]
        struct E {
            error: String,
        }

        (self.1, Json(E { error: self.0 })).into_response()
    }
}

impl APIError {
    pub fn new<T>(status: StatusCode, msg: String) -> Result<T, Self> {
        Err(Self(msg, status))
    }

    pub fn from<T>(msg: String) -> Result<T, Self> {
        Self::new::<T>(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
}
