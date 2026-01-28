use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use crate::{
    auth::{jwt::encode_jwt, password::verify_password}, 
    error::ApiError, 
    state::AppState
};

#[derive(Debug, Deserialize )]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {

    let record = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| ApiError::BadRequest)?
    .ok_or(ApiError::NotFound)?;

    verify_password(&record.password_hash, &payload.password)?;

    let token = encode_jwt(record.id)?;

    Ok(Json(LoginResponse { token }))
}