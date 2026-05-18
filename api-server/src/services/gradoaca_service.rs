use crate::models::personal::GradoAcademico;
use crate::repositories::gradoaca_repo;
use crate::middleware::error::ApiError;
use sqlx::MySqlPool;
use serde_json::{json, Value};
pub async fn upsert_gradoacademico(
    db: &MySqlPool,
    doc: &GradoAcademico,
) -> Result<(Option<Value>, &'static str, u64), ApiError> {
    let es_actualizacion = doc.id > 0;
    let accion = if es_actualizacion {
        "editar grado academico"
    } else {
        "agrega grado academico"
    };
    let actual = if es_actualizacion {
        gradoaca_repo::obtener_grado_por_id(db, doc.id)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error al buscar grado actual: {}", e)))?
    } else {
        None
    };
    let rows_affected = gradoaca_repo::upsert_grado(db, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error de base de datos: {}", e)))?;
    let mut diff = serde_json::Map::new();
    if let Some(antiguo) = actual {
        if antiguo.profesion != doc.profesion {
            diff.insert(
                "profesion".to_string(),
                json!({"antes": antiguo.profesion, "despues": doc.profesion}),
            );
        }
        if antiguo.universidad != doc.universidad {
            diff.insert(
                "universidad".to_string(),
                json!({"antes": antiguo.universidad, "despues": doc.universidad}),
            );
        }
        if antiguo.nivel_academico != doc.nivel_academico {
            diff.insert(
                "nivel_academico".to_string(),
                json!({"antes": antiguo.nivel_academico, "despues": doc.nivel_academico}),
            );
        }
    } else {
        diff.insert(
            "nuevo_grado".to_string(),
            serde_json::to_value(doc).unwrap_or_default(),
        );
    }
    let diff_value = if !diff.is_empty() {
        Some(Value::Object(diff))
    } else {
        None
    };
    Ok((diff_value, accion, rows_affected))
}
pub async fn eliminar_gradoa(
    db: &MySqlPool,
    id: i32,
) -> Result<(Option<Value>, &'static str, String), ApiError> {
    let grado = gradoaca_repo::obtener_grado_por_id(db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al capturar grado: {}", e)))?;
    let grado = match grado {
        Some(g) => g,
        None => return Err(ApiError::NotFound("Grado académico no encontrado".into())),
    };
    gradoaca_repo::eliminar_grado(db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar grado: {}", e)))?;
    let diff = json!({
        "objeto_eliminado": {
            "id": grado.id,
            "profesion": grado.profesion,
            "universidad": grado.universidad,
            "nivel_academico": grado.nivel_academico,
            "abrv": grado.abrv,
        }
    });
    Ok((Some(diff), "eliminar grado academico", grado.dni))
}
