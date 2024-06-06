use crate::db::query::task as TaskQuery;
use crate::handlers::get_req;
use crate::handlers::types::task as TaskTypes;
use crate::models::task::Task;
use crate::models::AuthUser;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Extension, Json, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::APIResponse;

pub async fn create_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    req: Result<Json<TaskTypes::CreateTaskRequest>, JsonRejection>,
) -> Result<APIResponse<Task>, APIError> {
    let req_task = get_req(req)?;
    let task = TaskQuery::insert_task(pool, user.id, req_task).await?;
    Ok(APIResponse::created(task))
}
