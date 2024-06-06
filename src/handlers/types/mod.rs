pub mod task;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

pub struct APIResponse<T>(StatusCode, Json<T>)
where
    T: serde::Serialize;

impl<T> IntoResponse for APIResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl<T> APIResponse<T>
where
    T: serde::Serialize,
{
    pub fn new(status: StatusCode, body: T) -> Self {
        Self(status, Json(body))
    }

    pub fn ok(body: T) -> Self {
        Self(StatusCode::OK, Json(body))
    }

    pub fn created(body: T) -> Self {
        Self(StatusCode::CREATED, Json(body))
    }

    pub fn accepted(body: T) -> Self {
        Self(StatusCode::ACCEPTED, Json(body))
    }

    pub fn no_content() -> StatusCode {
        StatusCode::NO_CONTENT
    }
}
