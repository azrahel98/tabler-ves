use crate::middleware::error::ApiError;
use crate::models::personal::DocumentoSindicato;
use crate::repositories::sindicato_repo;
use serde_json::{json, Value};
use sqlx::MySqlPool;
pub async fn agregar_sindicato(
    db: &MySqlPool,
    doc: &DocumentoSindicato,
) -> Result<Vec<(Option<Value>, &'static str, String)>, ApiError> {
    let mut tx = db.begin().await.map_err(|e| ApiError::InternalError(format!("Error iniciando transacción: {}", e)))?;
    let documento_id = sindicato_repo::insertar_documento(&mut tx, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar documento: {}", e)))?;
    let mut diffs = Vec::new();
    for vinculo in &doc.vinculos {
        sindicato_repo::insertar_vinculo_sindicato(&mut tx, vinculo.id_vinculo, doc.sindicato, documento_id)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error al insertar vínculo sindicato: {}", e)))?;
        let diff = serde_json::to_value(doc).unwrap_or_default();
        diffs.push((Some(diff), "afiliar al sindicato", vinculo.dni.clone()));
    }
    tx.commit().await.map_err(|e| ApiError::InternalError(format!("Error commiteando transacción: {}", e)))?;
    Ok(diffs)
}
pub async fn eliminar_sindicato(
    db: &MySqlPool,
    vinculo_id: i32,
    dni: &str,
    tipo: Option<&str>,
    numero: Option<i32>,
    year: Option<i32>,
    fecha: &str,
    fecha_valida: Option<&str>,
    descripcion: &str,
) -> Result<(Option<Value>, &'static str, String), ApiError> {
    let info = sindicato_repo::obtener_info_sindicato(db, vinculo_id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al capturar afiliación: {}", e)))?;
    let mut tx = db.begin().await
        .map_err(|e| ApiError::InternalError(format!("Error iniciando transacción: {}", e)))?;
    sindicato_repo::registrar_desafiliacion(&mut tx, vinculo_id, tipo, numero, year, fecha, fecha_valida, descripcion)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al desafiliar: {}", e)))?;
    tx.commit().await
        .map_err(|e| ApiError::InternalError(format!("Error al confirmar transacción: {}", e)))?;
    let diff = if let Some(i) = info {
        Some(json!({
            "vinculo_id": i.vinculo_id,
            "sindicato": i.sindicato_nombre,
            "documento_afiliacion_id": i.documento_afiliacion,
            "doc_desafiliacion": { "fecha": fecha, "descripcion": descripcion }
        }))
    } else {
        None
    };
    Ok((diff, "desafiliar sindicato", dni.to_string()))
}
