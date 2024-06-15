use super::{OFFSET, PAGE_LIMIT};
use crate::{errors::APIError, models::user as M};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn search(
    pool: PgPool,
    search_query: String,
    page: i16,
) -> Result<Vec<M::User>, APIError> {
    match sqlx::query_as::<_, M::User>(
        "
    SELECT id, username, name FROM users
    WHERE username ILIKE '%' || $1 || '%' OR name ILIKE '%' || $1 || '%' ORDER BY created_at LIMIT $2 OFFSET $3; 
    ",
    )
    .bind( search_query)
    .bind(PAGE_LIMIT)
    .bind(OFFSET(page))
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

pub async fn select_user_profile(pool: &PgPool, id: Uuid) -> Result<M::User, APIError> {
    match sqlx::query_as::<_, M::User>("SELECT id, username, name from users WHERE id = $1")
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

pub async fn select_received_requests(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<M::User>, APIError> {
    match sqlx::query_as::<_, M::User>(
        "
    SELECT u.id, u.username, u.name FROM users u
    INNER JOIN user_connection_requests r ON u.id = r.sender_id
    WHERE r.receiver_id = $1 ORDER BY r.sent_at DESC
    ;
    ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!("Failed to get received requests: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn select_sent_requests(pool: &PgPool, user_id: Uuid) -> Result<Vec<M::User>, APIError> {
    match sqlx::query_as::<_, M::User>(
        "
    SELECT u.id, u.username, u.name FROM users u
    INNER JOIN user_connection_requests r ON u.id = r.receiver_id
    WHERE r.sender_id = $1 ORDER BY r.sent_at DESC
    ;
    ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!("Failed to get sent requests: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn select_listers(
    pool: &PgPool,
    user_id: Uuid,
    page: i16,
) -> Result<Vec<M::User>, APIError> {
    match sqlx::query_as::<_, M::User>(
        "
        SELECT u.id, u.username, u.name FROM users u
        INNER JOIN user_connections c ON u.id = c.connected_id
        WHERE c.user_id = $1 ORDER BY u.connected_at ASC LIMIT $2 OFFSET $3;
        ",
    )
    .bind(user_id)
    .bind(PAGE_LIMIT)
    .bind(OFFSET(page))
    .fetch_all(pool)
    .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!("Failed to get listers: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn search_listers(
    pool: &PgPool,
    user_id: Uuid,
    search_query: String,
    page: i16,
) -> Result<Vec<M::User>, APIError> {
    match sqlx::query_as::<_, M::User>(
        "
        SELECT u.id, u.username, u.name FROM users u
        INNER JOIN user_connections c ON u.id = c.connected_id
        WHERE c.user_id = $1 AND (u.username ILIKE '%' || $2 || '%' OR u.name ILIKE '%' || $2 || '%') 
        ORDER BY u.connected_at ASC LIMIT $3 OFFSET $4;
        ",
    )
    .bind(user_id)
    .bind(search_query)
    .bind(PAGE_LIMIT)
    .bind(OFFSET(page))
    .fetch_all(pool)
    .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            tracing::error!("Failed to search listers: {:?}", e);
            Err(APIError::server())
        }
    }
}

pub async fn delete_connection(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
    connected_id: Uuid,
) -> Result<(), APIError> {
    match sqlx::query("DELETE FROM user_connections WHERE user_id = $1 AND connected_id = $2;")
        .bind(user_id)
        .bind(connected_id)
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
            tracing::error!("Failed to delete connection: {:?}", e);
            Err(APIError::server())
        }
    }
}
