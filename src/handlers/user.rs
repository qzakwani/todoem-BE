use crate::db::query::user as Q;
use crate::handlers::get_req;
use crate::handlers::types::user as T;
// use crate::models::AuthUser;
// use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::APIResponse;

pub async fn search(
    State(pool): State<PgPool>,
    Query(q): Query<String>,
    Query(p): Query<u8>,
) -> Result<APIResponse<Vec<T::UserSearchResponse>>, APIError> {
    if q.is_empty() {
        return Err(APIError::bad("Query parameter 'q' is required".to_string()));
    }

    let page = if p > 0 { p } else { 1 };
}
