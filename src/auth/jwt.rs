use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn encode_jwt(user_id: Uuid) -> Result<String, ApiError> {
    let secret = std::env::var("JWT_SECRET").unwrap();

    let claims = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .map_err(|_| ApiError::BadRequest)
    
}

pub fn decode_jwt(token: &str) -> Result<Claims, ApiError> {

    let secret = std::env::var("JWT_SECRET").unwrap();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map(|data| data.claims)
    .map_err(|_| ApiError::BadRequest)

}