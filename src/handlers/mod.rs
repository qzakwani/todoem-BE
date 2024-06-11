use crate::errors::APIError;
use axum::extract::Json;

pub mod task;
pub mod types;
pub mod user;

use axum::extract::rejection::JsonRejection;
use http::StatusCode;

fn get_req<T>(req: Result<Json<T>, JsonRejection>) -> Result<T, APIError> {
    match req {
        Ok(t) => {
            let res = t.0;
            Ok(res)
        }
        Err(JsonRejection::MissingJsonContentType(e)) => Err(APIError::new(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            &e.to_string(),
        )),
        Err(JsonRejection::JsonDataError(e)) => Err(APIError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            &e.to_string(),
        )),
        Err(JsonRejection::JsonSyntaxError(e)) => Err(APIError::bad(&e.to_string())),
        Err(JsonRejection::BytesRejection(_)) => Err(APIError::bad("Invalid JSON")),
        Err(e) => {
            tracing::error!("Unknown JSON rejection error {:#?}", e);
            Err(APIError::server())
        }
    }
}
