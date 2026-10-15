use crate::common::errors::ApiError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static JWT_SECRET: OnceLock<String> = OnceLock::new();

fn get_jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| {
        std::env::var("JWT_KEY").expect("JWT_KEY must be set")
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
    pub role: String,
    pub email: String,
    pub full_name: String,
    pub picture_url: Option<String>,
}

pub fn generate_token(
    user_id: i32,
    role: String,
    email: String,
    full_name: String,
    picture_url: Option<String>,
) -> Result<String, ApiError> {
    let secret_key = get_jwt_secret();
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());
    let claims = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        role,
        email,
        full_name,
        picture_url,
        id: user_id,
    };
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ApiError::InternalError(format!("Error generando token: {}", e)))
}

pub fn decodificar_token(token: &str) -> Result<Claims, ApiError> {
    let secret_key = get_jwt_secret();
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let data = decode::<Claims>(token, &decoding_key, &Validation::default())
        .map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;
    Ok(data.claims)
}
