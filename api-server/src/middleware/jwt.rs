use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
    pub lvl: Option<i32>,
    pub nombre: String,
}

pub fn generate_token(user_id: i32, level: i32, nombre: String) -> String {
    let secret_key = std::env::var("JWT_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    let claims = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        lvl: Some(level),
        nombre,
        id: user_id,
    };

    encode(&Header::default(), &claims, &encoding_key).unwrap()
}
