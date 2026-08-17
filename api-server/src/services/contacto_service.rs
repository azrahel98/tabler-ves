use crate::middleware::error::ApiError;
use crate::models::personal::ContactoEmergencia;
use crate::repositories::contacto_repo;
use serde_json::{Value, json};
use sqlx::MySqlPool;
pub async fn get_contacto_by_dni(
    db: &MySqlPool,
    dni: &str,
) -> Result<Option<ContactoEmergencia>, ApiError> {
    contacto_repo::get_contacto_by_dni(db, dni)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al obtener contacto de emergencia".into())
        })
}
pub async fn upsert_contacto(
    db: &MySqlPool,
    contacto: &ContactoEmergencia,
) -> Result<(Option<Value>, &'static str, u64), ApiError> {
    let actual = contacto_repo::get_contacto_by_dni(db, &contacto.persona_dni)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al buscar contacto actual: {}", e)))?;
    let rows_affected = contacto_repo::insert_or_update_contacto(db, contacto)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;
    let mut diff = serde_json::Map::new();
    let operacion: &'static str;
    if let Some(antiguo) = actual {
        operacion = "editar contacto de emergencia";
        if antiguo.nombre != contacto.nombre {
            diff.insert(
                "nombre".to_string(),
                json!({"antes": antiguo.nombre, "despues": contacto.nombre}),
            );
        }
        if antiguo.telefono != contacto.telefono {
            diff.insert(
                "telefono".to_string(),
                json!({"antes": antiguo.telefono, "despues": contacto.telefono}),
            );
        }
        if antiguo.relacion != contacto.relacion {
            diff.insert(
                "relacion".to_string(),
                json!({"antes": antiguo.relacion, "despues": contacto.relacion}),
            );
        }
    } else {
        operacion = "crear contacto de emergencia";
        diff.insert(
            "nuevo_contacto".to_string(),
            serde_json::to_value(contacto).unwrap_or_default(),
        );
    }
    let diff_value = if !diff.is_empty() {
        Some(Value::Object(diff))
    } else {
        None
    };
    Ok((diff_value, operacion, rows_affected))
}
pub async fn delete_contacto(
    db: &MySqlPool,
    dni: &str,
) -> Result<(Option<Value>, &'static str, String), ApiError> {
    let contacto = contacto_repo::get_contacto_by_dni(db, dni)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al capturar contacto: {}", e)))?;
    let _ = contacto_repo::delete_contacto(db, dni)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar contacto: {}", e)))?;
    let diff = if let Some(c) = contacto {
        Some(json!({
            "objeto_eliminado": {
                "nombre": c.nombre,
                "relacion": c.relacion,
                "telefono": c.telefono
            }
        }))
    } else {
        None
    };
    Ok((diff, "eliminar contacto de emergencia", dni.to_string()))
}
