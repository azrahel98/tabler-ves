use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificacionDto {
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

impl From<crate::domain::entities::notificacion::Notificacion> for NotificacionDto {
    fn from(n: crate::domain::entities::notificacion::Notificacion) -> Self {
        Self {
            id: n.id,
            tipo: n.tipo,
            titulo: n.titulo,
            mensaje: n.mensaje,
            avatar: n.avatar,
            enlace: n.enlace,
            leido: n.leido,
            metadata: n.metadata,
            created_at: n.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListarNotificacionesResponse {
    pub notificaciones: Vec<NotificacionDto>,
    pub no_leidas: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListarQuery {
    pub limit: Option<i64>,
}
