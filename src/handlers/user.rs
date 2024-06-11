use crate::db::query::user as Q;
// use crate::handlers::get_req;
use crate::handlers::types::user as T;
use crate::models::AuthUser;
// use axum::extract::rejection::JsonRejection;
use axum::extract::{Extension, Path, Query, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::{APIResponse, APISuccess};

pub async fn search(
    State(pool): State<PgPool>,
    Query(params): Query<T::SearchParams>,
) -> Result<APIResponse<Vec<T::UserSearchResponse>>, APIError> {
    if params.q.is_empty() {
        return Err(APIError::bad("Query parameter 'q' is required"));
    }

    let page = params.p.unwrap_or(1) as i16;

    let users = Q::search(pool, params.q, page).await?;

    Ok(APIResponse::ok(users))
}

pub async fn view_user_profile(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APIResponse<T::ViewUserResponse>, APIError> {
    if user.id != id {
        return Err(APIError::forbidden());
    }

    let mut profile = Q::select_user_profile(&pool, id).await?;
    profile.connected = Q::is_user_connected(&pool, user.id, id).await?;
    if profile.connected {
        profile.sent_connection = false;
        profile.received_connection = false;
    } else {
        profile.sent_connection = Q::is_connection_requested(&pool, user.id, id).await?;
        profile.received_connection = Q::is_connection_requested(&pool, id, user.id).await?;
    }

    Ok(APIResponse::ok(profile))
}

pub async fn request_connection(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::bad("You cannot connect with yourself"));
    }

    if Q::is_user_connected(&pool, user.id, id).await? {
        return Err(APIError::bad("You are already connected with this user"));
    }

    if Q::is_connection_requested(&pool, user.id, id).await? {
        return Err(APIError::bad(
            "You have already sent a connection request to this user",
        ));
    }

    if Q::is_connection_requested(&pool, id, user.id).await? {
        return Err(APIError::bad(
            "This user has already sent you a connection request",
        ));
    }

    Q::insert_request_connection(&pool, user.id, id).await?;

    Ok(APIResponse::ok_msg("User connection request sent"))
}

pub async fn delete_request_connection(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::bad("You cannot connect with yourself"));
    }

    if !Q::is_connection_requested(&pool, user.id, id).await? {
        return Err(APIError::bad(
            "You have not sent a connection request to this user",
        ));
    }

    Q::delete_request_connection(&pool, user.id, id).await?;

    Ok(APIResponse::ok_msg("User connection request deleted"))
}

pub async fn accept_connection(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::bad("You cannot connect with yourself"));
    }

    if Q::is_user_connected(&pool, user.id, id).await? {
        return Err(APIError::bad("You are already connected with this user"));
    }

    if !Q::is_connection_requested(&pool, id, user.id).await? {
        return Err(APIError::bad(
            "This user has not sent you a connection request",
        ));
    }

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {:?}", e);
            return Err(APIError::server());
        }
    };

    Q::insert_connection_tx(&mut tx, id, user.id).await?;
    Q::delete_request_connection_tx(&mut tx, id, user.id).await?;

    Ok(APIResponse::ok_msg("User connection request accepted"))
}

pub async fn reject_connection(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::bad("You cannot connect with yourself"));
    }

    if !Q::is_connection_requested(&pool, id, user.id).await? {
        return Err(APIError::bad(
            "This user has not sent you a connection request",
        ));
    }

    Q::delete_request_connection(&pool, id, user.id).await?;

    Ok(APIResponse::ok_msg("User connection request rejected"))
}
