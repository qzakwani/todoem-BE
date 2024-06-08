use crate::db::query::user as Q;
// use crate::handlers::get_req;
use crate::handlers::types::user as T;
// use crate::models::AuthUser;
// use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::APIResponse;

pub async fn search(
    State(pool): State<PgPool>,
    Query(params): Query<T::SearchParams>,
) -> Result<APIResponse<Vec<T::UserSearchResponse>>, APIError> {
    if params.q.is_empty() {
        return Err(APIError::bad("Query parameter 'q' is required".to_string()));
    }

    let page = params.p.unwrap_or(1) as i16;

    let users = Q::search(pool, params.q, page).await?;

    Ok(APIResponse::ok(users))
}
