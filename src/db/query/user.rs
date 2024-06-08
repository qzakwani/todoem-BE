use super::PAGE_LIMIT;
use crate::{errors::APIError, handlers::types::user as T};
use sqlx::PgPool;

pub async fn search(
    pool: PgPool,
    search_query: String,
    page: i16,
) -> Result<Vec<T::UserSearchResponse>, APIError> {
    let offset = (page - 1) * PAGE_LIMIT;
    match sqlx::query_as::<_, T::UserSearchResponse>(
        "
    SELECT id, username, name FROM users
    WHERE username ILIKE $1 OR name ILIKE $1 ORDER BY created_at LIMIT $2 OFFSET $3; 
    ",
    )
    .bind(format!("%{}%", search_query))
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
