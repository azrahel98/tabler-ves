use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notificacion {
    pub id: i32,
    pub tipo: String,
    pub titulo: String,
    pub mensaje: String,
    pub avatar: Option<String>,
    pub enlace: Option<String>,
    pub leido: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificacionEvento {
    pub id: i32,
    pub tipo: String,
    pub titulo: String,
    pub mensaje: String,
    pub avatar: Option<String>,
    pub enlace: Option<String>,
    pub leido: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
}
