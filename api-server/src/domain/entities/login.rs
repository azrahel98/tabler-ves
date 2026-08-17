use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Usuario {
    pub id: i32,
    pub google_sub: String,
    pub email: String,
    pub full_name: String,
    pub picture_url: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}





