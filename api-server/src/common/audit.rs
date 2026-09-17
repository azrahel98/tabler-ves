use crate::common::errors::ApiError;
use crate::common::jwt::Claims;
use actix_web::{HttpMessage, HttpRequest};
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct AuditContext {
    pub user_id: i32,
    pub ip: String,
    pub user_agent: String,
}

impl AuditContext {
    pub fn from_request(req: &HttpRequest) -> Self {
        Self {
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
        }
    }
}

pub fn get_db_key() -> &'static str {
    crate::keys::db_key()
}

pub async fn registrar_historial_db(
    db: &MySqlPool,
    audit_ctx: &AuditContext,
    operacion: &str,
    dni: &str,
    detalle: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    let detalle_final = serde_json::json!({
        "data": detalle.unwrap_or(serde_json::json!({})),
        "metadata": {
            "ip": audit_ctx.ip,
            "user_agent": audit_ctx.user_agent
        }
    })
    .to_string();
    let key = get_db_key();
    sqlx::query("CALL registrar_historial(?,?,?,?,?)")
        .bind(audit_ctx.user_id)
        .bind(operacion)
        .bind(detalle_final)
        .bind(key)
        .bind(dni)
        .execute(db)
        .await
        .map_err(|e| {
            eprintln!("❌ Error crítico al registrar historial: {:?}", e);
            ApiError::InternalError("Error al salvar log de auditoría".into())
        })?;
    Ok(())
}

pub async fn registrar_historial(
    req: &HttpRequest,
    db: &MySqlPool,
    operacion: &str,
    dni: &str,
    detalle: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    let audit_ctx = AuditContext::from_request(req);
    registrar_historial_db(db, &audit_ctx, operacion, dni, detalle).await
}
