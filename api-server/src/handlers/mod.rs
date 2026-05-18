pub mod dash;
pub mod login;
pub mod personal;
pub mod usuarios;
pub mod fileserver;
use crate::middleware::error::ApiError;
use actix_web::{HttpMessage, HttpRequest};
pub async fn registrar_historial(
    req: &HttpRequest,
    db: &sqlx::MySqlPool,
    operacion: &str,
    dni: &str,
    detalle: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    use crate::models::audit::AuditContext;
    use crate::middleware::jwt::Claims;
    let audit_ctx = AuditContext {
        user_id: req
            .extensions()
            .get::<Claims>()
            .map(|claims| claims.id)
            .unwrap_or(0),
        ip: req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string(),
        user_agent: req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string(),
    };
    crate::services::audit_service::registrar_historial(db, &audit_ctx, operacion, dni, detalle).await
}
