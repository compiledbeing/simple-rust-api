use axum::{body::Body, http::Request, middleware::Next, response::Response};
use crate::{auth::jwt::decode_jwt, error::ApiError};

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::BadRequest)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::BadRequest)?;

    let claims = decode_jwt(token)?;
    // store user id for downstream handlers (optional)
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
