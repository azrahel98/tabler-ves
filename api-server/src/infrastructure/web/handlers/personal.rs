use crate::AppState;
use crate::infrastructure::web::middleware::{
    error::{ApiError, validar},
    jwt::Claims,
};
use crate::infrastructure::web::models::personal::{
    ContactoEmergencia, DatosBancarios, DatosBancariosResponse, Documento,
    DocumentoSindicato, EventoVinculoPayload, NuevoVinculo,
    Perfil,
};
use crate::application::usecases::{
    banco_service, contacto_service, documento_service, personal_service,
    sindicato_service,
};
use actix_files::NamedFile;
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder,
    web::{self},
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::Row;
use validator::Validate;
use super::registrar_historial;
#[derive(Deserialize, Validate)]
pub struct PerfilDni {
    #[validate(custom(function = "crate::infrastructure::web::models::personal::es_dni_valido"))]
    pub dni: String,
}
pub async fn perfil_por_dni(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let trabajador = personal_service::get_perfil_by_dni(&data.db, &dni).await?;
    Ok(HttpResponse::Ok().json(trabajador))
}
#[derive(Deserialize, Validate)]
pub struct BuscarNombre {
    #[validate(length(min = 1, message = "El nombre no puede estar vacío"))]
    pub nombre: String,
}
pub async fn buscar_por_nombre(
    data: web::Data<AppState>,
    query: web::Query<BuscarNombre>,
) -> Result<impl Responder, ApiError> {
    validar(&query.0)?;
    let trabajador = personal_service::buscar_por_nombre(&data.db, &query.nombre).await?;
    Ok(HttpResponse::Ok().json(trabajador))
}
pub async fn editar_perfil(
    data: web::Data<AppState>,
    perfil: web::Json<Perfil>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&perfil.0)?;
    use crate::infrastructure::web::middleware::jwt::Claims;
    use crate::infrastructure::web::models::audit::AuditContext;
    use actix_web::HttpMessage;
    let audit_ctx = AuditContext {
        user_id: req
            .extensions()
            .get::<Claims>()
            .map(|claims| claims.id)
            .unwrap_or(0),
        ip: req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("origen desconocido")
            .to_string(),
        user_agent: req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("origen desconocido")
            .to_string(),
    };
    let perfil_entity = perfil.0.clone().into();
    let audit_ctx_entity = audit_ctx.clone().into();
    let rows_affected = personal_service::update_perfil(&data.db, &perfil_entity, &audit_ctx_entity).await?;
    Ok(HttpResponse::Ok().json(format!("Rows affected: {}", rows_affected)))
}
pub async fn consultar_dni_reniec(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let perfil =
        personal_service::consultar_dni_reniec(&data.db, &data.cliente_http, &dni).await?;
    Ok(HttpResponse::Ok().json(perfil))
}
pub async fn banco_por_dni(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let datos = banco_service::banco_por_dni(&data.db, &dni).await?;
    Ok(HttpResponse::Ok().json(datos))
}
pub async fn agregar_infobancaria(
    data: web::Data<AppState>,
    doc: web::Json<DatosBancariosResponse>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let doc_entity = doc.0.clone().into();
    let rows_affected = banco_service::agregar_infobancaria(&data.db, &doc_entity).await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "ingresar cuenta bancaria",
        &doc.dni,
        Some(serde_json::to_value(&doc.0).unwrap_or_default()),
    )
    .await;
    Ok(HttpResponse::Ok().json(format!("Rows affected: {}", rows_affected)))
}
pub async fn editar_datos_bancarios(
    data: web::Data<AppState>,
    doc: web::Json<DatosBancarios>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let doc_entity = doc.0.clone().into();
    let (diff_value, accion, rows_affected) =
        banco_service::editar_datos_bancarios(&data.db, &doc_entity).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &doc.dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json(format!("Rows affected: {}", rows_affected)))
}
pub async fn conctaco_por_dni(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let datos = contacto_service::get_contacto_by_dni(&data.db, &dni).await?;
    Ok(HttpResponse::Ok().json(datos))
}
pub async fn contacto_emergencia_add(
    data: web::Data<AppState>,
    contacto: web::Json<ContactoEmergencia>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&contacto.0)?;
    let contacto_entity = contacto.0.clone().into();
    let (diff_value, operacion, rows_affected) =
        contacto_service::upsert_contacto(&data.db, &contacto_entity).await?;
    if let Some(diff) = diff_value {
        let _ =
            registrar_historial(&req, &data.db, operacion, &contacto.persona_dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json(format!("Rows affected: {}", rows_affected)))
}
#[derive(Deserialize, Validate)]
pub struct EliminarContactoBody {
    #[validate(custom(function = "crate::infrastructure::web::models::personal::es_dni_valido"))]
    pub persona_dni: String,
}
pub async fn eliminar_contacto(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let (diff_value, accion, dni) =
        contacto_service::delete_contacto(&data.db, &id).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Contacto de emergencia eliminado"))
}
pub async fn agregar_sindicato(
    data: web::Data<AppState>,
    doc: web::Json<DocumentoSindicato>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let doc_entity = doc.0.clone().into();
    let diffs = sindicato_service::agregar_sindicato(&data.db, &doc_entity).await?;
    for (diff_value, accion, dni) in diffs {
        if let Some(diff) = diff_value {
            let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
        }
    }
    Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
}
#[derive(Deserialize, Validate)]
pub struct EliminarSindicatoBody {
    #[validate(range(min = 1, message = "ID de vínculo inválido"))]
    pub vinculo_id: i32,
    pub dni: String,
    #[serde(rename = "tipoDocumento")]
    pub tipo_documento: Option<String>,
    #[serde(rename = "numeroDocumento")]
    pub numero_documento: Option<i32>,
    #[serde(rename = "añoDocumento")]
    pub año_documento: Option<i32>,
    #[validate(length(min = 1, message = "La fecha es requerida"))]
    pub fecha: String,
    #[serde(rename = "fechaValida")]
    pub fecha_valida: Option<String>,
    #[validate(length(min = 1, message = "La descripción es requerida"))]
    pub descripcion: String,
}
pub async fn eliminar_sindicato(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<EliminarSindicatoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let id = path.into_inner();
    let (diff_value, accion, dni) = sindicato_service::eliminar_sindicato(
        &data.db,
        id,
        &body.dni,
        body.tipo_documento.as_deref(),
        body.numero_documento,
        body.año_documento,
        &body.fecha,
        body.fecha_valida.as_deref(),
        &body.descripcion,
    )
    .await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Afiliación sindical eliminada"))
}

pub async fn obtener_documento_por_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let doc_entity = documento_service::obtener_documento_por_id(&data.db, id).await?;
    let doc_web: crate::infrastructure::web::models::personal::Documento = doc_entity.into();
    Ok(HttpResponse::Ok().json(doc_web))
}

#[derive(Deserialize, Validate)]
pub struct CrearDocumentoBody {
    pub dni: Option<String>,
    #[validate(nested)]
    pub documento: Documento,
}

pub async fn crear_documento(
    data: web::Data<AppState>,
    body: web::Json<CrearDocumentoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let doc_entity = body.documento.clone().into();
    let new_id = documento_service::crear_documento(&data.db, &doc_entity).await?;
    if let Some(ref dni) = body.dni {
        let _ = registrar_historial(
            &req,
            &data.db,
            "crear documento",
            dni,
            Some(serde_json::json!({
                "id": new_id,
                "tipo": body.documento.tipo,
                "area_id": body.documento.area_id,
                "numero": body.documento.numero,
                "año": body.documento.año,
                "fecha": body.documento.fecha,
                "descripcion": body.documento.descripcion,
            })),
        )
        .await;
    }
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Documento creado correctamente",
        "id": new_id
    })))
}

#[derive(Deserialize, Validate)]
pub struct EditarDocumentoBody {
    #[validate(custom(function = "crate::infrastructure::web::models::personal::es_dni_valido"))]
    pub dni: String,
    pub documento: Documento,
}
pub async fn editar_documento(
    data: web::Data<AppState>,
    body: web::Json<EditarDocumentoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let id_documento = body.documento.id.unwrap_or(0);
    if id_documento == 0 {
        return Err(ApiError::BadRequest("ID de documento requerido".into()));
    }
    let documento_entity = body.documento.clone().into();
    let (diff_value, accion, dni) =
        documento_service::editar_documento(&data.db, id_documento, &documento_entity, &body.dni)
            .await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Documento actualizado"))
}

pub async fn eliminar_documento(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let doc_eliminado = documento_service::eliminar_documento(&data.db, id).await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "eliminar documento",
        "00000000",
        Some(serde_json::json!({
            "id": id,
            "tipo": doc_eliminado.tipo,
            "area_id": doc_eliminado.area_id,
            "numero": doc_eliminado.numero,
            "año": doc_eliminado.año,
            "descripcion": doc_eliminado.descripcion,
        })),
    )
    .await;
    Ok(HttpResponse::Ok().json("Documento eliminado correctamente"))
}
pub async fn registrar_trabajador(
    data: web::Data<AppState>,
    body: web::Json<NuevoVinculo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let key = crate::infrastructure::db::repositories::get_db_key();
    let mut tx = data.db.begin().await?;
    sqlx::query(
        r#"
        INSERT INTO persona (dni, nombre, apaterno, amaterno, telf1, direccion, email, ruc, fecha_nacimiento, sexo,region,distrito)
        VALUES (?, ?, ?, ?, aes_encrypt(?,?), aes_encrypt(?,?), aes_encrypt(?,?), ?, ?, ?,?,?)
        ON DUPLICATE KEY UPDATE
            nombre = VALUES(nombre),
            apaterno = VALUES(apaterno),
            amaterno = VALUES(amaterno),
            telf1 = VALUES(telf1),
            direccion = VALUES(direccion),
            email = VALUES(email),
            ruc = VALUES(ruc),
            fecha_nacimiento = VALUES(fecha_nacimiento),
            sexo = VALUES(sexo),
            region = VALUES(region),
            distrito = VALUES(distrito)
        "#,
    )
    .bind(&body.personal.dni)
    .bind(&body.personal.nombre)
    .bind(&body.personal.apaterno)
    .bind(&body.personal.amaterno)
    .bind(&body.personal.telf)
    .bind(&key)
    .bind(&body.personal.direccion)
    .bind(&key)
    .bind(&body.personal.email)
    .bind(&key)
    .bind(&body.personal.ruc)
    .bind(&body.personal.nacimiento)
    .bind(&body.personal.sexo)
    .bind(&body.personal.region)
    .bind(&body.personal.distrito)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Upsert Persona error: {}", e)))?;
    let doc_result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&body.documento.tipo)
    .bind(body.documento.area_id.unwrap_or(body.area))
    .bind(body.documento.numero)
    .bind(body.documento.año)
    .bind(&body.documento.fecha)
    .bind(&body.documento.fecha_valida)
    .bind(&body.documento.descripcion)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Documento error: {}", e)))?;
    let doc_id = doc_result.last_insert_id();
    sqlx::query(
        r#"
        INSERT INTO vinculo (dni, doc_ingreso_id, area_id, cargo_id, plaza_id, sueldo, estado,regimen)
        VALUES (?, ?, ?, ?, ?, ?, 'activo',?)
        "#,
    )
    .bind(&body.personal.dni)
    .bind(doc_id)
    .bind(body.area)
    .bind(body.cargo)
    .bind(&body.airshp)
    .bind(body.sueldo)
    .bind(body.regimen)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Vinculo error: {}", e)))?;
    sqlx::query(
        r#"
        UPDATE plaza SET estado = 'ocupado' WHERE codigo = ?
        "#,
    )
    .bind(&body.airshp)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Plaza error: {}", e)))?;
    tx.commit().await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "registrar nuevo trabajador",
        &body.personal.dni,
        Some(serde_json::to_value(&body.personal).unwrap_or_default()),
    )
    .await;
    Ok(HttpResponse::Ok().json("Trabajador registrado correctamente"))
}
pub async fn vinculos_por_dni(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let trabajador = personal_service::get_vinculos_by_dni(&data.db, &dni).await?;
    Ok(HttpResponse::Ok().json(trabajador))
}
#[derive(Deserialize, Validate)]
pub struct EliminarVinculoBody {
    #[validate(range(min = 1, message = "ID de vínculo inválido"))]
    pub id: i32,
}
pub async fn eliminar_vinculo(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let mut tx = data.db.begin().await?;
    let vinculo_full = sqlx::query!("SELECT * FROM vinculo WHERE id = ?", id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            ApiError::InternalError(format!("Error al capturar foto del vínculo: {}", e))
        })?;
    let dni: String = vinculo_full.dni.clone();
    let _doc_ingreso_id: Option<i64> = vinculo_full.doc_ingreso_id.map(|id| id as i64);
    let _doc_salida_id: Option<i64> = vinculo_full.doc_salida_id.map(|id| id as i64);
    let _plaza_id: Option<String> = vinculo_full.plaza_id.clone();
    sqlx::query("DELETE FROM vinculo_sindicato WHERE vinculo_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar sindicato: {}", e)))?;
    sqlx::query("DELETE FROM vinculo WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar vínculo: {}", e)))?;
    tx.commit().await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "eliminar vínculo",
        &dni,
        Some(serde_json::json!({
            "objeto_eliminado": {
                "id": vinculo_full.id,
                "dni": vinculo_full.dni,
                "plaza_id": vinculo_full.plaza_id,
                "sueldo": vinculo_full.sueldo,
                "estado": vinculo_full.estado,
                "area_id": vinculo_full.area_id,
                "cargo_id": vinculo_full.cargo_id,
                "regimen": vinculo_full.regimen
            }
        })),
    )
    .await;
    Ok(HttpResponse::Ok().json("Vínculo eliminado correctamente"))
}
pub async fn renuncia_por_vinculo(
    data: web::Data<AppState>,
    doc: web::Json<Documento>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;
    let insert_result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.area_id)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(&doc.descripcion)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Documento error: {}", e)))?;
    let new_doc_id = insert_result.last_insert_id();
    sqlx::query(
        r#"
        UPDATE vinculo
        SET estado = 'inactivo', doc_salida_id = ?
        WHERE id = ?
        "#,
    )
    .bind(new_doc_id)
    .bind(doc.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Vinculo error: {}", e)))?;
    sqlx::query("DELETE FROM vinculo_sindicato WHERE vinculo_id = ?")
        .bind(doc.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al limpiar sindicato: {}", e)))?;
    sqlx::query(
        r#"
        UPDATE plaza p
        JOIN vinculo v ON p.codigo = v.plaza_id
        SET p.estado = 'vacante'
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Plaza error: {}", e)))?;
    let row = sqlx::query(
        r#"
        SELECT
          v.dni,
          cr.nombre,
          v.estado,
          ds.fecha,
          ds.descripcion,
          CONCAT_WS('-', ds.tipo_documento_id, ds.numero, ds.year) AS documento
        FROM vinculo v
        INNER JOIN documento ds ON v.doc_salida_id = ds.id
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Select after update error: {}", e)))?;
    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;
    let json_value = serde_json::json!({
        "dni": row.try_get::<Option<String>, _>("dni")
            .map_err(|e| ApiError::InternalError(format!("Row get dni error: {}", e)))?,
        "nombre": row.try_get::<Option<String>, _>("nombre")
            .map_err(|e| ApiError::InternalError(format!("Row get nombre error: {}", e)))?,
        "estado": row.try_get::<Option<String>, _>("estado")
            .map_err(|e| ApiError::InternalError(format!("Row get estado error: {}", e)))?,
        "fecha": row.try_get::<Option<NaiveDate>, _>("fecha")
            .map_err(|e| ApiError::InternalError(format!("Row get fecha error: {}", e)))?
            .map(|d| d.to_string()),
        "descripcion": row.try_get::<Option<String>, _>("descripcion")
            .map_err(|e| ApiError::InternalError(format!("Row get descripcion error: {}", e)))?,
        "documento": row.try_get::<Option<String>, _>("documento")
            .map_err(|e| ApiError::InternalError(format!("Row get documento error: {}", e)))?,
    });
    if let Err(e) = registrar_historial(
        &req,
        &data.db,
        "registro de renuncia",
        json_value.get("dni").unwrap().as_str().unwrap(),
        Some(json_value.clone()),
    )
    .await
    {
        eprintln!("registrar_historial failed: {}", e);
    }
    Ok(HttpResponse::Ok().json(json_value))
}
pub async fn upsert_evento_vinculo(
    data: web::Data<AppState>,
    payload: web::Json<EventoVinculoPayload>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&payload.0)?;
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;
    if let Some(ref doc_inicio) = payload.documento_inicio {
        let doc_id = sqlx::query(
            r#"
            INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, descripcion)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doc_inicio.tipo)
        .bind(doc_inicio.area_id.or(payload.nueva_area_id))
        .bind(doc_inicio.numero)
        .bind(doc_inicio.año)
        .bind(&doc_inicio.fecha)
        .bind(&doc_inicio.fecha_valida)
        .bind(&doc_inicio.descripcion)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar documento de inicio: {}", e)))?
        .last_insert_id() as i32;
        sqlx::query(
            r#"
            INSERT INTO eventovinculo (vinculo_id, tipo_evento, nueva_area_id, documento_inicio, estado)
            VALUES (?, ?, ?, ?, 'activo')
            "#,
        )
        .bind(payload.vinculo_id)
        .bind(&payload.tipo_evento)
        .bind(payload.nueva_area_id)
        .bind(doc_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar evento: {}", e)))?;
        if payload.tipo_evento == "abandono" {
            sqlx::query("UPDATE vinculo SET estado = 'pendiente' WHERE id = ?")
                .bind(payload.vinculo_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    ApiError::InternalError(format!(
                        "Error al actualizar estado del vínculo: {}",
                        e
                    ))
                })?;
        }
    } else if let Some(ref doc_salida) = payload.documento_salida {
        let evento_id = payload.id.filter(|&id| id > 0).ok_or_else(|| {
            ApiError::BadRequest("Se requiere el id del evento para cerrarlo".into())
        })?;
        let doc_salida_id = sqlx::query(
            r#"
            INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, descripcion)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doc_salida.tipo)
        .bind(doc_salida.area_id)
        .bind(doc_salida.numero)
        .bind(doc_salida.año)
        .bind(&doc_salida.fecha)
        .bind(&doc_salida.fecha_valida)
        .bind(&doc_salida.descripcion)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar documento de salida: {}", e)))?
        .last_insert_id() as i32;
        sqlx::query(
            r#"
            UPDATE eventovinculo
            SET documento_salida = ?, estado = 'desactivado'
            WHERE id = ?
            "#,
        )
        .bind(doc_salida_id)
        .bind(evento_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al cerrar evento: {}", e)))?;
        sqlx::query("UPDATE vinculo SET estado = 'activo' WHERE id = ?")
            .bind(payload.vinculo_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ApiError::InternalError(format!("Error al reactivar el vínculo: {}", e))
            })?;
    } else {
        return Err(ApiError::BadRequest(
            "Se debe enviar documento_inicio o documento_salida".into(),
        ));
    }
    let row = sqlx::query(
        r#"
        SELECT
            pe.dni,
            CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS persona_nombre,
            cr.nombre AS cargo,
            ar.nombre AS area
        FROM vinculo v
        INNER JOIN persona pe ON v.dni = pe.dni
        LEFT JOIN cargo cr ON v.cargo_id = cr.id
        LEFT JOIN area ar ON v.area_id = ar.id
        WHERE v.id = ?
        "#,
    )
    .bind(payload.vinculo_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error fetching vinculo info: {}", e)))?;
    let dni: String = row.try_get("dni").unwrap_or_default();
    let persona_nombre: String = row.try_get("persona_nombre").unwrap_or_default();
    let cargo: String = row.try_get("cargo").unwrap_or_default();
    let area: String = row.try_get("area").unwrap_or_default();
    let doc_desc = if let Some(ref doc) = payload.documento_inicio {
        doc.descripcion.clone()
    } else if let Some(ref doc) = payload.documento_salida {
        doc.descripcion.clone()
    } else {
        String::new()
    };
    let accion = if payload.documento_inicio.is_some() {
        format!("registrar evento {}", payload.tipo_evento)
    } else {
        format!("cerrar evento {}", payload.tipo_evento)
    };
    let payload_historico = serde_json::json!({
        "dni": dni,
        "nombre": persona_nombre,
        "cargo": cargo,
        "area": area,
        "tipo_evento": payload.tipo_evento,
        "descripcion": doc_desc,
    });
    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;
    let _ = registrar_historial(&req, &data.db, &accion, &dni, Some(payload_historico)).await;
    Ok(HttpResponse::Ok().json("Operación exitosa"))
}
#[derive(Deserialize, Validate)]
pub struct DeleteEventoVinculoBody {
    #[validate(range(min = 1, message = "ID de evento inválido"))]
    pub id: i32,
}
pub async fn delete_evento_vinculo(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB error: {}", e)))?;
    let evento_full = sqlx::query!(
        "SELECT ev.*, pe.dni 
         FROM eventovinculo ev 
         INNER JOIN vinculo v ON ev.vinculo_id = v.id 
         INNER JOIN persona pe ON v.dni = pe.dni 
         WHERE ev.id = ?",
        id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error capturando evento: {}", e)))?;
    sqlx::query("DELETE FROM eventovinculo WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error eliminando evento: {}", e)))?;
    sqlx::query("DELETE FROM documento WHERE id = ?")
        .bind(evento_full.documento_inicio)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error eliminando documento: {}", e)))?;
    if let Some(ds) = evento_full.documento_salida {
        sqlx::query("DELETE FROM documento WHERE id = ?")
            .bind(ds)
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error eliminando doc salida: {}", e)))?;
    }
    let dni = evento_full.dni.clone();
    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "eliminar evento de legajo",
        &dni,
        Some(serde_json::json!({
            "objeto_eliminado": {
                "id": evento_full.id,
                "tipo": evento_full.tipo_evento,
                "vinculo_id": evento_full.vinculo_id,
                "estado": evento_full.estado
            }
        })),
    )
    .await;
    Ok(HttpResponse::Ok().json("Evento de vínculo eliminado"))
}
#[derive(Deserialize, Validate)]
pub struct CambioAreaPayload {
    #[validate(range(min = 1, message = "ID de vínculo inválido"))]
    pub vinculo_id: i32,
    #[validate(range(min = 1, message = "ID de área inválido"))]
    pub nueva_area_id: i32,
    #[validate(length(min = 1, message = "La fecha del cambio es requerida"))]
    pub fecha_cambio: String,
    #[validate(nested)]
    pub documento: Documento,
}
pub async fn registrar_cambio_area(
    data: web::Data<AppState>,
    payload: web::Json<CambioAreaPayload>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&payload.0)?;
    let user_id = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.id)
        .filter(|&id| id > 0);
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;
    let vinculo = sqlx::query!(
        r#"
        SELECT v.id, v.dni, v.area_id, v.estado, a.nombre AS area_nombre
        FROM vinculo v
        INNER JOIN area a ON v.area_id = a.id
        WHERE v.id = ?
        "#,
        payload.vinculo_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al buscar vínculo: {}", e)))?
    .ok_or_else(|| ApiError::NotFound("Vínculo no encontrado".into()))?;
    if vinculo.estado != "activo" {
        return Err(ApiError::BadRequest(format!(
            "El vínculo no está activo (estado actual: {})",
            vinculo.estado
        )));
    }
    if vinculo.area_id == payload.nueva_area_id {
        return Err(ApiError::BadRequest(
            "El vínculo ya pertenece a esa área".into(),
        ));
    }
    let nueva_area = sqlx::query!(
        "SELECT id, nombre FROM area WHERE id = ?",
        payload.nueva_area_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al buscar área nueva: {}", e)))?
    .ok_or_else(|| ApiError::NotFound("Área nueva no encontrada".into()))?;
    let rotacion_activa = sqlx::query!(
        r#"
        SELECT id FROM eventovinculo
        WHERE vinculo_id = ? AND tipo_evento = 'rotacion' AND estado = 'activo'
        LIMIT 1
        "#,
        payload.vinculo_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al verificar rotación: {}", e)))?;
    if rotacion_activa.is_some() {
        return Err(ApiError::BadRequest(
            "El vínculo tiene una rotación activa. Cierre la rotación antes de cambiar el área."
                .into(),
        ));
    }
    let documento_id = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, area_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.documento.tipo)
    .bind(payload.documento.area_id.unwrap_or(payload.nueva_area_id))
    .bind(payload.documento.numero)
    .bind(payload.documento.año)
    .bind(&payload.documento.fecha)
    .bind(&payload.documento.fecha_valida)
    .bind(&payload.documento.descripcion)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al insertar documento: {}", e)))?
    .last_insert_id() as i32;
    sqlx::query(
        r#"
        INSERT INTO cambio_area
            (vinculo_id, area_anterior_id, area_nueva_id, documento_id, fecha_cambio, user_id)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(payload.vinculo_id)
    .bind(vinculo.area_id)
    .bind(payload.nueva_area_id)
    .bind(documento_id)
    .bind(&payload.fecha_cambio)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al insertar cambio de área: {}", e)))?;
    sqlx::query("UPDATE vinculo SET area_id = ? WHERE id = ?")
        .bind(payload.nueva_area_id)
        .bind(payload.vinculo_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al actualizar vínculo: {}", e)))?;
    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "cambio de area",
        &vinculo.dni,
        Some(json!({
            "vinculo_id": payload.vinculo_id,
            "area_anterior": { "id": vinculo.area_id, "nombre": vinculo.area_nombre },
            "area_nueva":    { "id": nueva_area.id,   "nombre": nueva_area.nombre },
            "fecha_cambio": payload.fecha_cambio,
            "documento": {
                "tipo": payload.documento.tipo,
                "numero": payload.documento.numero,
                "año": payload.documento.año,
                "fecha": payload.documento.fecha,
                "descripcion": payload.documento.descripcion,
            }
        })),
    )
    .await;
    Ok(HttpResponse::Ok().json("Cambio de área registrado correctamente"))
}
#[derive(Deserialize, serde::Serialize)]
pub struct Vacante {
    pub id: Option<i32>,
    pub dni: Option<String>,
    pub nombre: Option<String>,
    pub fecha: Option<NaiveDate>,
    pub fechavalida: Option<NaiveDate>,
    pub area: Option<String>,
    pub cargo: Option<String>,
    pub codigo: Option<String>,
    pub sueldo: Option<f64>,
    pub area_id: Option<i32>,
    pub cargo_id: Option<i32>,
    pub avatar: Option<String>,
}
pub async fn buscar_vacantes(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let vacantes = sqlx::query_as!(
        Vacante,
        r#"
        SELECT
        v.id,
        v.dni,
        CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS nombre,
        d.fecha,
        d.fecha_valida AS fechavalida,
        ar.nombre AS area,
        ar.id as area_id,
        cr.nombre AS cargo,
        cr.id as cargo_id,
        pl.codigo,
        v.sueldo,
        pe.avatar
        FROM plaza AS pl
        LEFT JOIN vinculo AS v ON pl.codigo = v.plaza_id
        LEFT JOIN documento AS d ON v.doc_salida_id = d.id
        LEFT JOIN persona AS pe ON v.dni = pe.dni
        LEFT JOIN area AS ar ON v.area_id = ar.id
        LEFT JOIN cargo AS cr ON v.cargo_id = cr.id
        WHERE 
        pl.estado = 'vacante'
        ORDER BY 
        d.fecha DESC;
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database error retrieving vacantes".into())
    })?;
    Ok(HttpResponse::Ok().json(vacantes))
}
#[derive(Deserialize)]
pub struct BuscarPlaza {
    pub codigo: String,
}
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PlazaDetalle {
    pub codigo: String,
    pub cargo_estructural: Option<String>,
    pub cargo_descripcion: Option<String>,
    pub grupo_ocupacional: Option<String>,
    pub grupo_descripcion: Option<String>,
    pub condicion: Option<String>,
    pub regimen_id: i32,
    pub regimen: String,
}
pub async fn buscar_por_plaza(
    data: web::Data<AppState>,
    query: web::Query<BuscarPlaza>,
) -> Result<impl Responder, ApiError> {
    let plaza = sqlx::query_as::<_, PlazaDetalle>(
        r#"
        select
        p.codigo,
        ce.codigo cargo_estructural,
        ce.descripcion cargo_descripcion,
        go.codigo grupo_ocupacional,
        go.descripcion grupo_descripcion,
        p.condicion,
        r.id regimen_id,
        r.decreto regimen
        from
        plaza p
        inner join cargoestructural ce on p.cargoestructural = ce.codigo
        inner join gruposocupacionales go on p.grupoocupacional = go.codigo
        inner join regimen r on p.regimen = r.id
        where p.codigo = ?
        "#,
    )
    .bind(query.codigo.clone())
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database error retrieving plaza".into())
    })?;
    Ok(HttpResponse::Ok().json(plaza))
}
pub async fn buscar_areas(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let areas =
        sqlx::query("SELECT id, nombre, activo, nivel, sigla FROM area WHERE activo = 1 ORDER BY nombre")
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                eprintln!("Database error: {:?}", e);
                ApiError::InternalError("Error al obtener áreas".into())
            })?;
    let resultado: Vec<Value> = areas
        .iter()
        .map(|row| {
            let activo = row
                .try_get::<bool, _>("activo")
                .unwrap_or_else(|_| row.try_get::<i8, _>("activo").map(|v| v != 0).unwrap_or(true));
            json!({
                "id": row.get::<i32, _>("id"),
                "nombre": row.get::<String, _>("nombre"),
                "activo": activo,
                "nivel": row.try_get::<Option<i32>, _>("nivel").unwrap_or_default(),
                "sigla": row.try_get::<Option<String>, _>("sigla").unwrap_or_default(),
            })
        })
        .collect();
    Ok(HttpResponse::Ok().json(resultado))
}
pub async fn buscar_cargos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let cargos =
        sqlx::query("SELECT id, nombre,activo FROM cargo where activo = true ORDER BY nombre")
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                eprintln!("Database error: {:?}", e);
                ApiError::InternalError("Error al obtener cargos".into())
            })?;
    let resultado: Vec<Value> = cargos
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "nombre": row.get::<String, _>("nombre"),
                "activo": row.get::<bool, _>("activo"),
            })
        })
        .collect();
    Ok(HttpResponse::Ok().json(resultado))
}
#[derive(Deserialize, Validate)]
pub struct ActivosPorDistritoBody {
    #[validate(length(min = 1, message = "El distrito es requerido"))]
    pub distrito: String,
}
pub async fn activos_por_distrito(
    data: web::Data<AppState>,
    query: web::Query<ActivosPorDistritoBody>,
) -> Result<impl Responder, ApiError> {
    validar(&query.0)?;
    let key = crate::infrastructure::db::repositories::get_db_key();
    let distrito = query.distrito.trim();
    let sin_asignar = distrito.eq_ignore_ascii_case("SIN ASIGNAR");

    let base_personas_query = r#"
SELECT
  CAST(p.dni AS CHAR)                                             AS dni,
  CONCAT(p.apaterno, ' ', p.amaterno, ' ', p.nombre)              AS nombre,
  dc.fecha                                                        AS ingreso,
  cast(aes_decrypt(p.direccion,?) as char)                        AS direccion,
  ar.id                                                           AS area_id,
  ar.nombre                                                       AS area_nombre,
  cr.id                                                           AS cargo_id,
  cr.nombre                                                       AS cargo_nombre,
  s.nombre                                                        AS sindicato,
  rg.id                                                           AS regimen_id,
  rg.nombre                                                       AS regimen_nombre,
  COALESCE(NULLIF(p.distrito, ''), 'SIN ASIGNAR')                 AS distrito,
  p.avatar                                                        AS avatar
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
  INNER JOIN cargo cr ON v.cargo_id = cr.id
  INNER JOIN area ar ON v.area_id = ar.id
  INNER JOIN documento dc ON v.doc_ingreso_id = dc.id
  INNER JOIN regimen rg ON v.regimen = rg.id
  LEFT JOIN documento dcs ON v.doc_salida_id = dcs.id
  LEFT JOIN vinculo_sindicato vs ON vs.vinculo_id = v.id
  LEFT JOIN sindicato s ON vs.sindicato_id = s.id
WHERE
  v.estado = 'activo'
"#;

    let filas_personas = if sin_asignar {
        sqlx::query(&format!(
            "{base_personas_query} AND (p.distrito IS NULL OR p.distrito = '')"
        ))
        .bind(&key)
        .fetch_all(&data.db)
        .await
    } else {
        sqlx::query(&format!(
            "{base_personas_query} AND UPPER(p.distrito) = UPPER(?)"
        ))
        .bind(&key)
        .bind(distrito)
        .fetch_all(&data.db)
        .await
    }
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener activos por distrito".into())
    })?;

    let base_areas_query = r#"
SELECT
  ar.id                                                           AS id,
  ar.nombre                                                       AS nombre,
  COUNT(*)                                                        AS cantidad
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
  INNER JOIN area ar ON v.area_id = ar.id
WHERE
  v.estado = 'activo'
"#;

    let filas_areas = if sin_asignar {
        sqlx::query(&format!(
            "{base_areas_query} AND (p.distrito IS NULL OR p.distrito = '') GROUP BY ar.id, ar.nombre ORDER BY cantidad DESC, ar.nombre ASC"
        ))
        .fetch_all(&data.db)
        .await
    } else {
        sqlx::query(&format!(
            "{base_areas_query} AND UPPER(p.distrito) = UPPER(?) GROUP BY ar.id, ar.nombre ORDER BY cantidad DESC, ar.nombre ASC"
        ))
        .bind(distrito)
        .fetch_all(&data.db)
        .await
    }
    .map_err(|e| {
        eprintln!("Database error al obtener áreas por distrito: {:?}", e);
        ApiError::InternalError("Error al obtener áreas por distrito".into())
    })?;

    let base_edad_query = r#"
SELECT
  CASE
    WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 18 AND 25 THEN '18-25'
    WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 26 AND 35 THEN '26-35'
    WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 36 AND 45 THEN '36-45'
    WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 46 AND 55 THEN '46-55'
    ELSE '55+'
  END AS nombre,
  COUNT(*) AS cantidad
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
WHERE
  v.estado = 'activo'
"#;

    let filas_edad = if sin_asignar {
        sqlx::query(&format!(
            "{base_edad_query} AND (p.distrito IS NULL OR p.distrito = '') GROUP BY 1 ORDER BY MIN(TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE))"
        ))
        .fetch_all(&data.db)
        .await
    } else {
        sqlx::query(&format!(
            "{base_edad_query} AND UPPER(p.distrito) = UPPER(?) GROUP BY 1 ORDER BY MIN(TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE))"
        ))
        .bind(distrito)
        .fetch_all(&data.db)
        .await
    }
    .map_err(|e| {
        eprintln!("Database error al obtener rangos de edad por distrito: {:?}", e);
        ApiError::InternalError("Error al obtener rangos de edad por distrito".into())
    })?;

    let personas: Vec<Value> = filas_personas
        .iter()
        .map(|row| {
            let ingreso: Option<NaiveDate> = row.try_get("ingreso").ok();
            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "ingreso": ingreso.map(|d| d.to_string()),
                "direccion": row.try_get::<Option<String>, _>("direccion").unwrap_or(None),
                "area": {
                    "id": row.get::<i32, _>("area_id"),
                    "nombre": row.get::<String, _>("area_nombre"),
                },
                "cargo": {
                    "id": row.get::<i32, _>("cargo_id"),
                    "nombre": row.get::<String, _>("cargo_nombre"),
                },
                "regimen": {
                    "id": row.get::<i32, _>("regimen_id"),
                    "nombre": row.get::<String, _>("regimen_nombre"),
                },
                "sindicato": row.try_get::<Option<String>, _>("sindicato").unwrap_or(None),
                "distrito": row.get::<String, _>("distrito"),
                "avatar": row.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();

    let areas: Vec<Value> = filas_areas
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "nombre": row.get::<String, _>("nombre"),
                "cantidad": row.get::<i64, _>("cantidad"),
            })
        })
        .collect();

    let rangos_edad: Vec<Value> = filas_edad
        .iter()
        .map(|row| {
            json!({
                "nombre": row.get::<String, _>("nombre"),
                "cantidad": row.get::<i64, _>("cantidad"),
            })
        })
        .collect();

    let total = personas.len();

    let resultado = json!({
        "distrito": if sin_asignar { "SIN ASIGNAR" } else { distrito },
        "total": total,
        "areas": areas,
        "rangos_edad": rangos_edad,
        "personas": personas,
    });

    Ok(HttpResponse::Ok().json(resultado))
}
pub async fn calidad_datos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let key = crate::infrastructure::db::repositories::get_db_key();
    let filas_domicilio = sqlx::query(
        r#"
        SELECT DISTINCT
          CAST(p.dni AS CHAR) AS dni,
          CONCAT(p.apaterno, ' ', p.amaterno, ' ', p.nombre) AS nombre,
          CAST(AES_DECRYPT(p.direccion, ?) AS CHAR) AS direccion,
          p.distrito
        FROM persona p
        INNER JOIN vinculo v ON v.dni = p.dni AND v.estado = 'activo'
        WHERE (p.distrito IS NULL OR p.distrito = ''
           OR p.direccion IS NULL
           OR CAST(AES_DECRYPT(p.direccion, ?) AS CHAR) IS NULL
           OR CAST(AES_DECRYPT(p.direccion, ?) AS CHAR) = '')
        GROUP BY p.dni
        ORDER BY nombre
        "#,
    )
    .bind(&key)
    .bind(&key)
    .bind(&key)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error sin_domicilio: {}", e)))?;
    println!("filas_domicilio: {:?}", filas_domicilio);
    let sin_domicilio: Vec<Value> = filas_domicilio
        .iter()
        .map(|row| {
            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "direccion": row.try_get::<Option<String>, _>("direccion").unwrap_or(None),
                "distrito": row.try_get::<Option<String>, _>("distrito").unwrap_or(None),
            })
        })
        .collect();
    let filas_sin_salida = sqlx::query(
        r#"
        SELECT
          v.id,
          CAST(v.dni AS CHAR) AS dni,
          CONCAT(p.apaterno, ' ', p.amaterno, ' ', p.nombre) AS nombre,
          cr.nombre AS cargo,
          ar.nombre AS area
        FROM vinculo v
        INNER JOIN persona p ON p.dni = v.dni
        INNER JOIN cargo cr ON cr.id = v.cargo_id
        INNER JOIN area ar ON ar.id = v.area_id
        WHERE v.estado = 'inactivo' AND v.doc_salida_id IS NULL
        ORDER BY nombre
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error sin_documento_salida: {}", e)))?;
    let sin_documento_salida: Vec<Value> = filas_sin_salida
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "cargo": row.get::<String, _>("cargo"),
                "area": row.get::<String, _>("area"),
            })
        })
        .collect();
    Ok(HttpResponse::Ok().json(json!({
        "sin_domicilio": sin_domicilio,
        "sin_documento_salida": sin_documento_salida,
    })))
}

#[derive(Deserialize, Validate)]
pub struct AvatarPayload {
    #[validate(custom(function = "crate::infrastructure::web::models::personal::es_dni_valido"))]
    pub dni: String,
    #[validate(length(min = 1, max = 3_000_000, message = "Imagen inválida o demasiado grande"))]
    pub imagen_base64: String,
}

pub async fn subir_avatar(
    data: web::Data<AppState>,
    body: web::Json<AvatarPayload>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let avatar_url =
        personal_service::guardar_avatar(&data.db, &body.dni, &body.imagen_base64).await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "actualizar avatar",
        &body.dni,
        Some(serde_json::json!({ "avatar": avatar_url })),
    )
    .await;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "avatar": avatar_url })))
}

pub async fn ver_avatar(
    path: web::Path<String>,
) -> Result<NamedFile, ApiError> {
    let dni = path.into_inner();
    if dni.len() != 8 || !dni.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::BadRequest("DNI inválido".into()));
    }
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let file_path = std::path::PathBuf::from(upload_dir)
        .join("AVATARS")
        .join(format!("{}.png", dni));
    if !file_path.exists() {
        return Err(ApiError::NotFound("Avatar no encontrado".into()));
    }
    NamedFile::open(file_path)
        .map_err(|e| ApiError::InternalError(format!("Error abriendo archivo: {}", e)))
}
