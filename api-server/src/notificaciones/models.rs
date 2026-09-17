use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notificacion {
    pub id: i32,
    pub tipo: String,
    pub titulo: String,
    pub mensaje: String,
    pub dni: Option<String>,
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
    pub dni: Option<String>,
    pub leido: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListarNotificacionesResponse {
    pub notificaciones: Vec<Notificacion>,
    pub no_leidas: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListarQuery {
    pub limit: Option<i64>,
}
