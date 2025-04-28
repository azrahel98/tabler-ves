pub mod dash;
pub mod login;
pub mod personal;
use crate::middleware::{error::ApiError, jwt::Claims};
use actix_web::{HttpMessage, HttpRequest};
use sqlx::query;

pub async fn registrar_historial(
    req: &HttpRequest,
    db: &sqlx::MySqlPool,
    operacion: &str,
    detalle: Option<&str>,
) -> Result<(), ApiError> {
    let detalle_str = detalle.unwrap_or("");

    let user_id = req
        .extensions()
        .get::<Claims>()
        .map(|claims| claims.id)
        .unwrap_or(0);

    let _ = query!(
        r#"
        CALL registrar_historial(?, ?, ?)
        "#,
        user_id,
        operacion,
        detalle_str
    )
    .execute(db)
    .await
    .map_err(|e| {
        eprintln!("Error al registrar historial (ignorado): {:?}", e);
    });

    Ok(())
}
