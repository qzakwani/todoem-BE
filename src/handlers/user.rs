use super::types::user as T;
use crate::db::query::user as Q;
use crate::models::user as M;
use crate::models::AuthUser;
use axum::extract::{Extension, Path, Query, State};
use sqlx::PgPool;

use crate::errors::APIError;

use super::types::{APIResponse, APISuccess};

pub async fn search(
    State(pool): State<PgPool>,
    Query(params): Query<T::SearchParams>,
) -> Result<APIResponse<Vec<M::User>>, APIError> {
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
) -> Result<APIResponse<T::ViewUser>, APIError> {
    if user.id == id {
        return Err(APIError::forbidden());
    }

    let _user = Q::select_user_profile(&pool, id).await?;
    let mut profile = T::ViewUser::from(_user);
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
        return Err(APIError::forbidden());
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
        return Err(APIError::forbidden());
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
        return Err(APIError::forbidden());
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

    match tx.commit().await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to commit transaction: {:?}", e);
            return Err(APIError::server());
        }
    }

    Ok(APIResponse::ok_msg("User connection request accepted"))
}

pub async fn reject_connection(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::forbidden());
    }

    if !Q::is_connection_requested(&pool, id, user.id).await? {
        return Err(APIError::bad(
            "This user has not sent you a connection request",
        ));
    }

    Q::delete_request_connection(&pool, id, user.id).await?;

    Ok(APIResponse::ok_msg("User connection request rejected"))
}

pub async fn get_received_requests(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse<Vec<M::User>>, APIError> {
    let users = Q::select_received_requests(&pool, user.id).await?;
    Ok(APIResponse::ok(users))
}

pub async fn get_sent_requests(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<APIResponse<Vec<M::User>>, APIError> {
    let users = Q::select_sent_requests(&pool, user.id).await?;
    Ok(APIResponse::ok(users))
}

pub async fn get_listers(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(page): Path<u16>,
) -> Result<APIResponse<Vec<M::User>>, APIError> {
    let users = Q::select_listers(&pool, user.id, page as i16).await?;
    Ok(APIResponse::ok(users))
}

pub async fn search_listers(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Query(params): Query<T::SearchParams>,
) -> Result<APIResponse<Vec<M::User>>, APIError> {
    if params.q.is_empty() {
        return Err(APIError::bad("Query parameter 'q' is required"));
    }

    let page = params.p.unwrap_or(1) as i16;

    let users = Q::search_listers(&pool, user.id, params.q, page).await?;

    Ok(APIResponse::ok(users))
}

pub async fn view_lister_profile(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APIResponse<T::ViewUser>, APIError> {
    if user.id == id {
        return Err(APIError::forbidden());
    }

    let _user = Q::select_user_profile(&pool, id).await?;

    let mut profile = T::ViewUser::from(_user);
    profile.connected = true;
    profile.sent_connection = false;
    profile.received_connection = false;

    Ok(APIResponse::ok(profile))
}

pub async fn disconnect_lister(
    Extension(user): Extension<AuthUser>,
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<APISuccess, APIError> {
    if user.id == id {
        return Err(APIError::forbidden());
    }

    if !Q::is_user_connected(&pool, user.id, id).await? {
        return Err(APIError::bad("You are not connected with this user"));
    }

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {:?}", e);
            return Err(APIError::server());
        }
    };

    Q::delete_connection(&mut tx, user.id, id).await?;
    Q::delete_connection(&mut tx, id, user.id).await?;

    match tx.commit().await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to commit transaction: {:?}", e);
            return Err(APIError::server());
        }
    }

    Ok(APIResponse::ok_msg("User disconnected"))
}
