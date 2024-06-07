use crate::db::query::task as Q;
use crate::handlers::get_req;
use crate::handlers::types::task as T;
use crate::models::task::Task;
use crate::models::AuthUser;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Extension, Json, Path, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::APIResponse;

pub async fn create_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    req: Result<Json<T::CreateTaskRequest>, JsonRejection>,
) -> Result<APIResponse<Task>, APIError> {
    let req_task = get_req(req)?;
    let task = Q::insert_task(pool, user.id, req_task).await?;
    Ok(APIResponse::created(task))
}

pub async fn get_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<APIResponse<Task>, APIError> {
    let task = Q::select_task(pool, user.id, id).await?;
    Ok(APIResponse::ok(task))
}

pub async fn update_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    req: Result<Json<T::UpdateTaskRequest>, JsonRejection>,
) -> Result<APIResponse, APIError> {
    let req_task = get_req(req)?;
    Q::update_task(pool, user.id, id, req_task).await?;
    Ok(APIResponse::no_content())
}

pub async fn delete_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<APIResponse, APIError> {
    Q::delete_task(pool, user.id, id).await?;
    Ok(APIResponse::no_content())
}

pub async fn done_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<APIResponse, APIError> {
    Q::done_task(pool, user.id, id).await?;
    Ok(APIResponse::no_content())
}

pub async fn undone_task(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<APIResponse, APIError> {
    Q::undone_task(pool, user.id, id).await?;
    Ok(APIResponse::no_content())
}

pub async fn get_all_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse<Vec<Task>>, APIError> {
    let tasks = Q::select_all_tasks(pool, user.id).await?;
    Ok(APIResponse::ok(tasks))
}

pub async fn get_all_done_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse<Vec<Task>>, APIError> {
    let tasks = Q::select_all_tasks_by_status(pool, user.id, true).await?;
    Ok(APIResponse::ok(tasks))
}

pub async fn get_all_undone_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse<Vec<Task>>, APIError> {
    let tasks = Q::select_all_tasks_by_status(pool, user.id, false).await?;
    Ok(APIResponse::ok(tasks))
}

pub async fn delete_all_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse, APIError> {
    Q::delete_all_tasks(pool, user.id).await?;
    Ok(APIResponse::no_content())
}

pub async fn delete_all_done_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse, APIError> {
    Q::delete_all_tasks_by_status(pool, user.id, true).await?;
    Ok(APIResponse::no_content())
}

pub async fn delete_all_undone_tasks(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse, APIError> {
    Q::delete_all_tasks_by_status(pool, user.id, false).await?;
    Ok(APIResponse::no_content())
}
