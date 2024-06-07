use sqlx::PgPool;

use crate::{
    errors::APIError,
    handlers::types::task::{CreateTaskRequest, UpdateTaskRequest},
    models::task::Task,
};

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

pub async fn select_task(
    pool: PgPool,
    user_id: uuid::Uuid,
    task_id: i64,
) -> Result<Task, APIError> {
    match sqlx::query_as::<_, Task>(
        "
    SELECT * FROM tasks WHERE id = $1 AND user_id = $2 LIMIT 1;
    ",
    )
    .bind(task_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    {
        Ok(task) => Ok(task),
        Err(e) => {
            if matches!(e, sqlx::Error::RowNotFound) {
                return Err(APIError::not_found());
            }
            tracing::error!("Failed to select task: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn update_task(
    pool: PgPool,
    user_id: uuid::Uuid,
    task_id: i64,
    task: UpdateTaskRequest,
) -> Result<(), APIError> {
    match sqlx::query(
        "
    UPDATE tasks SET task = $1, description = $2, due_date = $3, repeat_frequency = $4
    WHERE id = $5 AND user_id = $6;
    ",
    )
    .bind(task.task)
    .bind(task.description)
    .bind(task.due_date)
    .bind(task.repeat_frequency)
    .bind(task_id)
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(row) => {
            if row.rows_affected() == 0 {
                return Err(APIError::not_found());
            }
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Failed to update task: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_task(pool: PgPool, user_id: uuid::Uuid, task_id: i64) -> Result<(), APIError> {
    match sqlx::query(
        "
    DELETE FROM tasks WHERE id = $1 AND user_id = $2;
    ",
    )
    .bind(task_id)
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(row) => {
            if row.rows_affected() == 0 {
                return Err(APIError::not_found());
            }
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Failed to delete task: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn done_task(pool: PgPool, user_id: uuid::Uuid, task_id: i64) -> Result<(), APIError> {
    match sqlx::query(
        "
    UPDATE tasks SET done = true WHERE id = $1 AND user_id = $2;
    ",
    )
    .bind(task_id)
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(row) => {
            if row.rows_affected() == 0 {
                return Err(APIError::not_found());
            }
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Failed to mark task as done: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn undone_task(pool: PgPool, user_id: uuid::Uuid, task_id: i64) -> Result<(), APIError> {
    match sqlx::query(
        "
    UPDATE tasks SET done = false WHERE id = $1 AND user_id = $2;
    ",
    )
    .bind(task_id)
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(row) => {
            if row.rows_affected() == 0 {
                return Err(APIError::not_found());
            }
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Failed to mark task as undone: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn select_all_tasks(pool: PgPool, user_id: uuid::Uuid) -> Result<Vec<Task>, APIError> {
    match sqlx::query_as::<_, Task>(
        "
    SELECT * FROM tasks WHERE user_id = $1 ORDER BY id DESC LIMIT 100;
    ",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    {
        Ok(tasks) => Ok(tasks),
        Err(e) => {
            tracing::error!("Failed to select all tasks: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn select_all_tasks_by_status(
    pool: PgPool,
    user_id: uuid::Uuid,
    done: bool,
) -> Result<Vec<Task>, APIError> {
    match sqlx::query_as::<_, Task>(
        "
    SELECT * FROM tasks WHERE user_id = $1 AND done = $2 ORDER BY id DESC LIMIT 100;
    ",
    )
    .bind(user_id)
    .bind(done)
    .fetch_all(&pool)
    .await
    {
        Ok(tasks) => Ok(tasks),
        Err(e) => {
            tracing::error!("Failed to select all done tasks: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_all_tasks(pool: PgPool, user_id: uuid::Uuid) -> Result<(), APIError> {
    match sqlx::query(
        "
    DELETE FROM tasks WHERE user_id = $1;
    ",
    )
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to delete all tasks: {:#?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_all_tasks_by_status(
    pool: PgPool,
    user_id: uuid::Uuid,
    done: bool,
) -> Result<(), APIError> {
    match sqlx::query(
        "
    DELETE FROM tasks WHERE user_id = $1 AND done = $2;
    ",
    )
    .bind(user_id)
    .bind(done)
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to delete all tasks by status: {:#?}", e);
            Err(APIError::server())
        }
    }
}
