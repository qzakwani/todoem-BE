pub mod task;
pub mod user;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

pub struct APIResponse<T = ()>(StatusCode, Option<Json<T>>)
where
    T: serde::Serialize;

impl<T> IntoResponse for APIResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match self.1 {
            Some(body) => (self.0, body).into_response(),
            None => self.0.into_response(),
        }
    }
}

impl<T> APIResponse<T>
where
    T: serde::Serialize,
{
    pub fn new(status: StatusCode, body: T) -> Self {
        Self(status, Some(Json(body)))
    }

    pub fn ok(body: T) -> Self {
        Self(StatusCode::OK, Some(Json(body)))
    }

    pub fn created(body: T) -> Self {
        Self(StatusCode::CREATED, Some(Json(body)))
    }

    pub fn accepted(body: T) -> Self {
        Self(StatusCode::ACCEPTED, Some(Json(body)))
    }

    pub fn status(status: StatusCode) -> Self {
        Self(status, None)
    }

    pub fn no_content() -> Self {
        Self::status(StatusCode::NO_CONTENT)
    }
}

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    msg: String,
}

pub type APISuccess = APIResponse<SuccessResponse>;

impl APISuccess {
    pub fn ok_msg(msg: &str) -> Self {
        APIResponse(
            StatusCode::OK,
            Some(Json(SuccessResponse {
                msg: msg.to_string(),
            })),
        )
    }
}
