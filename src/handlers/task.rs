use crate::handlers::types::task as TaskTypes;
use crate::models::task::Task;
use crate::models::AuthUser;
use axum::extract::{Extension, Json, State};
use axum::http::StatusCode;
use sqlx::PgPool;

use super::types::APIError;

pub async fn create_task(
    Extension(user): Extension<AuthUser>,
    Json(task): Json<TaskTypes::CreateTaskRequest>,
    State(pool): State<PgPool>,
) -> Result<StatusCode, APIError> {
    let new_task = sqlx::query_as!(
        Task,
        "
    INSERT INTO tasks (user_id, task, description, due_date, repeat_frequency)
    VALUES ($1, $2, $3, $4, $5)
    ",
        user.id,
        task.task,
        task.description,
        task.due_date,
        task.repeat_frequency
    )
    .fetch_one(&pool)
    .await?;

    println!("Created task: {:#?}", new_task);

    Ok(StatusCode::CREATED)
}
