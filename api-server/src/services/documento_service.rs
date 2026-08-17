use crate::middleware::error::ApiError;
use crate::models::personal::Documento;
use crate::repositories::documento_repo;
use serde_json::{json, Value};
use sqlx::MySqlPool;
pub async fn editar_documento(
    db: &MySqlPool,
    id: i32,
    doc: &Documento,
    dni: &str,
) -> Result<(Option<Value>, &'static str, String), ApiError> {
    let actual = documento_repo::obtener_documento_por_id(db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al buscar documento actual: {}", e)))?;
    let mut diff = serde_json::Map::new();
    if let Some(antiguo) = actual {
        if antiguo.tipo != doc.tipo {
            diff.insert(
                "tipo".to_string(),
                json!({"antes": antiguo.tipo, "despues": doc.tipo}),
            );
        }
        if antiguo.numero != doc.numero {
            diff.insert(
                "numero".to_string(),
                json!({"antes": antiguo.numero, "despues": doc.numero}),
            );
        }
        if antiguo.año != doc.año {
            diff.insert(
                "año".to_string(),
                json!({"antes": antiguo.año, "despues": doc.año}),
            );
        }
        if antiguo.fecha != doc.fecha {
            diff.insert(
                "fecha".to_string(),
                json!({"antes": antiguo.fecha, "despues": doc.fecha}),
            );
        }
        if antiguo.fecha_valida != doc.fecha_valida {
            diff.insert(
                "fecha_valida".to_string(),
                json!({"antes": antiguo.fecha_valida, "despues": doc.fecha_valida}),
            );
        }
        if antiguo.descripcion != doc.descripcion {
            diff.insert(
                "descripcion".to_string(),
                json!({"antes": antiguo.descripcion, "despues": doc.descripcion}),
            );
        }
    } else {
        return Err(ApiError::NotFound("Documento no encontrado".into()));
    }
    let _rows = documento_repo::actualizar_documento(db, id, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al actualizar documento: {}", e)))?;
    let diff_value = if !diff.is_empty() {
        Some(Value::Object(diff))
    } else {
        None
    };
    Ok((diff_value, "editar documento", dni.to_string()))
}
