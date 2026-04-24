pub mod dash;
pub mod login;
pub mod personal;
pub mod usuarios;

use crate::middleware::{error::ApiError, jwt::Claims};
use actix_web::{HttpMessage, HttpRequest};

pub async fn registrar_historial(
    req: &HttpRequest,
    db: &sqlx::MySqlPool,
    operacion: &str,
    dni: &str,
    detalle: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    
    let detalle_final = serde_json::json!({
        "data": detalle.unwrap_or(serde_json::json!({})),
        "metadata": {
            "ip": ip,
            "user_agent": user_agent
        }
    }).to_string();

    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");

    let user_id = req
        .extensions()
        .get::<Claims>()
        .map(|claims| claims.id)
        .unwrap_or(0);

    
    sqlx::query("CALL registrar_historial(?,?,?,?,?)")
        .bind(user_id)
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
}pub mod fileserver;
