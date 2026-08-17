use crate::middleware::error::ApiError;
use crate::models::personal::{DatosBancarios, DatosBancariosResponse};
use crate::repositories::banco_repo;
use serde_json::{Value, json};
use sqlx::MySqlPool;
pub async fn banco_por_dni(db: &MySqlPool, dni: &str) -> Result<Option<DatosBancarios>, ApiError> {
    banco_repo::banco_por_dni(db, dni).await.map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener datos bancarios".into())
    })
}
pub async fn agregar_infobancaria(
    db: &MySqlPool,
    doc: &DatosBancariosResponse,
) -> Result<u64, ApiError> {
    banco_repo::agregar_infobancaria(db, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))
}
pub async fn editar_datos_bancarios(
    db: &MySqlPool,
    doc: &DatosBancarios,
) -> Result<(Option<Value>, &'static str, u64), ApiError> {
    let actual = banco_repo::obtener_cuenta_por_id(db, doc.id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener cuenta actual: {}", e)))?;
    let rows_affected = banco_repo::actualizar_datos_bancarios(db, doc)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;
    let banco_id: i32 = doc.banco.parse().unwrap_or(0);
    let nombre_banco_nuevo = banco_repo::obtener_nombre_banco(db, banco_id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener banco: {}", e)))?;
    let mut diff = serde_json::Map::new();
    let tipo_nuevo = doc.tipo_cuenta.clone().unwrap_or_default().to_uppercase();
    if actual.numero_cuenta != doc.numero_cuenta {
        diff.insert(
            "numero_cuenta".to_string(),
            json!({"antes": actual.numero_cuenta, "despues": doc.numero_cuenta}),
        );
    }
    if actual.tipo_cuenta != Some(tipo_nuevo.clone()) {
        diff.insert(
            "tipo_cuenta".to_string(),
            json!({"antes": actual.tipo_cuenta, "despues": tipo_nuevo}),
        );
    }
    if actual.cci != doc.cci {
        diff.insert(
            "cci".to_string(),
            json!({"antes": actual.cci, "despues": doc.cci}),
        );
    }
    if actual.banco != nombre_banco_nuevo {
        diff.insert(
            "banco".to_string(),
            json!({"antes": actual.banco, "despues": nombre_banco_nuevo}),
        );
    }
    if actual.estado != doc.estado {
        diff.insert(
            "estado".to_string(),
            json!({"antes": actual.estado, "despues": doc.estado}),
        );
    }
    let diff_value = if !diff.is_empty() {
        Some(Value::Object(diff))
    } else {
        None
    };
    Ok((diff_value, "actualizar cuenta bancaria", rows_affected))
}
