use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

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
) -> String {
    let secret_key = std::env::var("JWT_KEY").expect("JWT_KEY must be set");
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());
    let claims = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        role,
        email,
        full_name,
        picture_url,
        id: user_id,
    };
    encode(&Header::default(), &claims, &encoding_key).unwrap()
}

