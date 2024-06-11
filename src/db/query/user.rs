use super::PAGE_LIMIT;
use crate::{errors::APIError, handlers::types::user as T};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn search(
    pool: PgPool,
    search_query: String,
    page: i16,
) -> Result<Vec<T::UserSearchResponse>, APIError> {
    let offset = (page - 1) * PAGE_LIMIT;
    match sqlx::query_as::<_, T::UserSearchResponse>(
        "
    SELECT id, username, name FROM users
    WHERE username ILIKE '%' || $1 || '%' OR name ILIKE '%' || $1 || '%' ORDER BY created_at LIMIT $2 OFFSET $3; 
    ",
    )
    .bind( search_query)
    .bind(PAGE_LIMIT)
    .bind(offset)
    .fetch_all(&pool)
    .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!("Failed to search for users: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn select_user_profile(pool: &PgPool, id: Uuid) -> Result<T::ViewUserResponse, APIError> {
    match sqlx::query_as::<_, T::ViewUserResponse>(
        "SELECT id, username, name from users WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            if matches!(e, sqlx::Error::RowNotFound) {
                return Err(APIError::not_found());
            }
            tracing::error!("Failed to select user profile: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn is_user_connected(
    pool: &PgPool,
    user_id: Uuid,
    connected_id: Uuid,
) -> Result<bool, APIError> {
    match sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM user_connections WHERE user_id = $1 AND connected_id = $2);",
    )
    .bind(user_id)
    .bind(connected_id)
    .fetch_one(pool)
    .await
    {
        Ok(i) => Ok(i),
        Err(e) => {
            tracing::error!("Failed to check if users are connected: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn is_connection_requested(
    pool: &PgPool,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<bool, APIError> {
    match sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM user_connection_requests WHERE sender_id = $1 AND receiver_id = $2);",
    )
    .bind(sender_id)
    .bind(receiver_id)
    .fetch_one(pool)
    .await
    {
        Ok(i) => Ok(i),
        Err(e) => {
            tracing::error!("Failed to check if connection is requested: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn insert_request_connection(
    pool: &PgPool,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<(), APIError> {
    match sqlx::query(
        "INSERT INTO user_connection_requests (sender_id, receiver_id) VALUES ($1, $2);",
    )
    .bind(sender_id)
    .bind(receiver_id)
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to request connection: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_request_connection(
    pool: &PgPool,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<(), APIError> {
    match sqlx::query(
        "DELETE FROM user_connection_requests WHERE sender_id = $1 AND receiver_id = $2;",
    )
    .bind(sender_id)
    .bind(receiver_id)
    .execute(pool)
    .await
    {
        Ok(r) => {
            if r.rows_affected() != 1 {
                return Err(APIError::not_found());
            }
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to delete connection request: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_request_connection_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<(), APIError> {
    match sqlx::query(
        "DELETE FROM user_connection_requests WHERE sender_id = $1 AND receiver_id = $2;",
    )
    .bind(sender_id)
    .bind(receiver_id)
    .execute(&mut **tx)
    .await
    {
        Ok(r) => {
            if r.rows_affected() != 1 {
                return Err(APIError::not_found());
            }
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to delete connection request: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn insert_connection_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sender_id: Uuid,
    receiver_id: Uuid,
) -> Result<(), APIError> {
    match sqlx::query(
        "INSERT INTO user_connections (user_id, connected_id) VALUES ($1, $2), ($2, $1);",
    )
    .bind(sender_id)
    .bind(receiver_id)
    .execute(&mut **tx)
    .await
    {
        Ok(r) => {
            if r.rows_affected() != 1 {
                return Err(APIError::not_found());
            }
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to insert connection: {:?}", e);
            Err(APIError::server())
        }
    }
}
