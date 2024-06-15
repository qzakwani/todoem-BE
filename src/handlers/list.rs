use crate::db::query::list as Q;
use crate::handlers::get_req;
use crate::handlers::types::list as T;
use crate::models::list::List;
use crate::models::AuthUser;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Extension, Json, Path, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::APIResponse;

pub async fn create_list(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    req: Json<T::CreateListRequest>,
) -> Result<APIResponse<List>, APIError> {
    todo!()
}
