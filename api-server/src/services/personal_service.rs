use crate::middleware::error::ApiError;
use crate::models::personal::{Perfil, PerfilInput, Persona, Vinculos};
use chrono::NaiveDate;
use reqwest::Client;
use serde_json::Value;
use crate::repositories::personal_repo;
use sqlx::MySqlPool;
pub async fn buscar_por_nombre(db: &MySqlPool, nombre: &str) -> Result<Vec<Persona>, ApiError> {
    let nombre_like = format!("%{}%", nombre);
    personal_repo::buscar_por_nombre(db, &nombre_like)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Database consulta malformada".into())
        })
}
pub async fn get_perfil_by_dni(db: &MySqlPool, dni: &str) -> Result<Perfil, ApiError> {
    personal_repo::get_perfil_by_dni(db, dni)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al obtener perfil".into())
        })
}
pub async fn get_vinculos_by_dni(db: &MySqlPool, dni: &str) -> Result<Vec<Vinculos>, ApiError> {
    personal_repo::get_vinculos_by_dni(db, dni)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al obtener vínculos".into())
        })
}
use crate::models::audit::AuditContext;
use crate::services::audit_service;
use serde_json::json;
pub async fn update_perfil(
    db: &MySqlPool,
    perfil: &Perfil,
    audit_ctx: &AuditContext,
) -> Result<u64, ApiError> {
    let perfil_antes = get_perfil_by_dni(db, &perfil.dni).await?;
    let rows_affected = personal_repo::update_perfil(db, perfil)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al actualizar perfil".into())
        })?;
    let mut diff = serde_json::Map::new();
    if perfil_antes.telf != perfil.telf {
        diff.insert(
            "telf".to_string(),
            json!({"antes": perfil_antes.telf, "despues": perfil.telf}),
        );
    }
    if perfil_antes.direccion != perfil.direccion {
        diff.insert(
            "direccion".to_string(),
            json!({"antes": perfil_antes.direccion, "despues": perfil.direccion}),
        );
    }
    if perfil_antes.email != perfil.email {
        diff.insert(
            "email".to_string(),
            json!({"antes": perfil_antes.email, "despues": perfil.email}),
        );
    }
    if perfil_antes.ruc != perfil.ruc {
        diff.insert(
            "ruc".to_string(),
            json!({"antes": perfil_antes.ruc, "despues": perfil.ruc}),
        );
    }
    if perfil_antes.region != perfil.region {
        diff.insert(
            "region".to_string(),
            json!({"antes": perfil_antes.region, "despues": perfil.region}),
        );
    }
    if perfil_antes.distrito != perfil.distrito {
        diff.insert(
            "distrito".to_string(),
            json!({"antes": perfil_antes.distrito, "despues": perfil.distrito}),
        );
    }
    if !diff.is_empty() {
        let _ = audit_service::registrar_historial(
            db,
            audit_ctx,
            "editar informacion personal",
            &perfil.dni,
            Some(serde_json::Value::Object(diff)),
        )
        .await;
    }
    Ok(rows_affected)
}
pub async fn consultar_dni_reniec(
    db: &MySqlPool,
    cliente_http: &Client,
    dni: &str,
) -> Result<PerfilInput, ApiError> {
    let local = personal_repo::consultar_dni_local(db, dni)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al consultar persona local".into())
        })?;
    if let Some(perfil) = local {
        return Ok(perfil);
    }
    let token = std::env::var("APINET")
        .map_err(|_| ApiError::InternalError("APINET no configurado".into()))?;
    let response = cliente_http
        .get("https://api.decolecta.com/v1/reniec/dni")
        .query(&[("numero", dni)])
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al consultar RENIEC: {}", e)))?;
    if !response.status().is_success() {
        return Err(ApiError::InternalError(format!(
            "RENIEC respondió con error: {}",
            response.status()
        )));
    }
    let api_data: Value = response
        .json()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al parsear respuesta: {}", e)))?;
    let nacimiento_str = api_data["fechaNacimiento"].as_str().unwrap_or("1900-01-01");
    let nacimiento = NaiveDate::parse_from_str(nacimiento_str, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(nacimiento_str, "%d/%m/%Y"))
        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
    let perfil = PerfilInput {
        dni: dni.to_string(),
        apaterno: api_data["first_last_name"].as_str().unwrap_or("").to_string(),
        amaterno: api_data["second_last_name"].as_str().unwrap_or("").to_string(),
        nombre: api_data["first_name"].as_str().unwrap_or("").to_string(),
        telf: None,
        direccion: None,
        email: None,
        ruc: None,
        nacimiento,
        sexo: None,
        region: None,
        distrito: None,
    };
    Ok(perfil)
}
