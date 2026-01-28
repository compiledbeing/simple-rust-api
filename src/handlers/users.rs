use axum::{
    Extension, 
    Json, 
    extract::{Path, State}, 
    http::StatusCode,  
};
use uuid::Uuid;

use crate::{
    auth::password::hash_password, 
    error::ApiError, 
    models::user::{CreateUser, User}, 
    state::AppState
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), ApiError> {

    let password_hash = hash_password(&payload.password)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, name, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, created_at
        "#,
        Uuid::new_v4(),
        payload.name,
        payload.email,
        password_hash
    ).fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {

    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    Ok(Json(users))
}


pub async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<User>, ApiError> {

    let user: User = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(user))
}


pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {

    let result = sqlx::query(
        "DELETE FROM users WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}


pub async fn greet_user(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<String>, ApiError> {

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?;

    Ok(Json(format!(
        "Welcome to a protected route {}!",
        user.name
    )))
}