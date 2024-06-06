use sqlx::PgPool;

use crate::{errors::APIError, handlers::types::task::CreateTaskRequest, models::task::Task};

pub async fn insert_task(
    pool: PgPool,
    user_id: uuid::Uuid,
    task: CreateTaskRequest,
) -> Result<Task, APIError> {
    match sqlx::query_as::<_, Task>(
        "
    INSERT INTO tasks (user_id, task, description, due_date, repeat_frequency)
    VALUES ($1, $2, $3, $4, $5) RETURNING *;
    ",
    )
    .bind(user_id)
    .bind(task.task)
    .bind(task.description)
    .bind(task.due_date)
    .bind(task.repeat_frequency)
    .fetch_one(&pool)
    .await
    {
        Ok(task) => Ok(task),
        Err(e) => {
            tracing::error!("Failed to insert task: {:#?}", e);
            Err(APIError::server())
        }
    }
}
