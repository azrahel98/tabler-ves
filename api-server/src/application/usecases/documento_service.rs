use crate::infrastructure::web::middleware::error::ApiError;
use crate::domain::entities::personal::Documento;
use crate::infrastructure::db::repositories::documento_repo;
use serde_json::{json, Value};
use sqlx::MySqlPool;
pub async fn obtener_documento_por_id(
    db: &MySqlPool,
    id: i32,
) -> Result<Documento, ApiError> {
    let actual = documento_repo::obtener_documento_por_id(db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al buscar documento: {}", e)))?;
    actual.ok_or_else(|| ApiError::NotFound("Documento no encontrado".into()))
}

pub async fn crear_documento(
    db: &MySqlPool,
    doc: &Documento,
) -> Result<u64, ApiError> {
    documento_repo::crear_documento(db, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al crear documento: {}", e)))
}

pub async fn eliminar_documento(
    db: &MySqlPool,
    id: i32,
) -> Result<Documento, ApiError> {
    let actual = documento_repo::obtener_documento_por_id(db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al buscar documento: {}", e)))?;
    let doc = actual.ok_or_else(|| ApiError::NotFound("Documento no encontrado".into()))?;
    let rows = documento_repo::eliminar_documento(db, id)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.is_foreign_key_violation() {
                    return ApiError::BadRequest(
                        "No se puede eliminar el documento porque está referenciado por otros registros".into(),
                    );
                }
            }
            ApiError::InternalError(format!("Error al eliminar documento: {}", e))
        })?;
    if rows == 0 {
        return Err(ApiError::NotFound("Documento no encontrado".into()));
    }
    Ok(doc)
}

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
        if antiguo.area_id != doc.area_id {
            diff.insert(
                "area_id".to_string(),
                json!({"antes": antiguo.area_id, "despues": doc.area_id}),
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
        if antiguo.conv != doc.conv {
            diff.insert(
                "conv".to_string(),
                json!({"antes": antiguo.conv, "despues": doc.conv}),
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
