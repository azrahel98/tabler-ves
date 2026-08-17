use crate::middleware::error::ApiError;
use crate::models::audit::AuditContext;
use sqlx::MySqlPool;
pub async fn registrar_historial(
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
    let key = crate::repositories::get_db_key();
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
