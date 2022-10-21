use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::{config::CONFIG, errors::ApiError};


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub id: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(id: String, permissions: Vec<String>) -> Self {
        Self {
            id,
            permissions,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn encode_jwt(claims: Claims) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> Result<Claims, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))
}
