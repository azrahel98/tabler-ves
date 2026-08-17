use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
pub struct Usuario {
    pub id: i32,
    pub google_sub: String,
    pub email: String,
    pub full_name: String,
    pub picture_url: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

