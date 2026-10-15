use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use futures_util::TryStreamExt;

use crate::common::errors::ApiError;
use crate::dash::service as dash_service;
use crate::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistorialQuery {
    pub dni: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    #[allow(dead_code)]
    pub key: Option<String>,
}

#[derive(Deserialize)]
pub struct AreaQuery {
    area_id: i32,
}

#[derive(Deserialize)]
pub struct SindicatoQuery {
    pub sindicato_id: Option<i32>,
    pub sindicato: Option<String>,
}

#[derive(Deserialize)]
pub struct RegimenQuery {
    pub regimen_id: Option<i32>,
    pub regimen: Option<String>,
}

#[get("/cumpleanos")]
pub async fn cumpleanos(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let list = dash_service::cumpleaños(&data.db).await?;
    Ok(HttpResponse::Ok().json(list))
}

#[get("/resumen")]
pub async fn info(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::info(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/areareport")]
pub async fn personal_area_report(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_area_report(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/renunciasano")]
pub async fn renuncias_ano(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::renuncias_ano(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/bancosreport")]
pub async fn bancos_report(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::bancos_report(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos")]
pub async fn reporte_personal_activo(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_personal_activo(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/area")]
pub async fn personal_activo_area(
    data: web::Data<AppState>,
    query: web::Query<AreaQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_activo_area(&data.db, query.area_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/sindicato")]
pub async fn personal_activo_sindicato(
    data: web::Data<AppState>,
    query: web::Query<SindicatoQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_activo_sindicato(
        &data.db,
        query.sindicato_id,
        query.sindicato.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/regimen")]
pub async fn personal_activo_regimen(
    data: web::Data<AppState>,
    query: web::Query<RegimenQuery>,
) -> Result<HttpResponse, ApiError> {
    let result =
        dash_service::personal_activo_regimen(&data.db, query.regimen_id, query.regimen.as_deref())
            .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/historial")]
pub async fn reporte_historial(
    data: web::Data<AppState>,
    query: web::Query<HistorialQuery>,
) -> Result<HttpResponse, ApiError> {
    let dni_filter = query.dni.as_deref().filter(|d| !d.trim().is_empty());
    let result =
        dash_service::reporte_historial(&data.db, dni_filter, query.page, query.limit, query.offset)
            .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/organigrama")]
pub async fn organigrama(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::organigrama(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/report-renuncia")]
pub async fn report_renuncias(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::report_renuncias(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/documentos")]
pub async fn reporte_documentos(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::tipos_documento(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/tipos_documento")]
pub async fn tipos_documento(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::tipos_documento(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/exportar_excel")]
pub async fn exportar_excel(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let buffer = dash_service::exportar_excel(&data.db).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"reporte_airhsp.xlsx\"",
        ))
        .body(buffer))
}

#[get("/activos/distrito")]
pub async fn activos_por_distrito(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::activos_por_distrito(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/trabajadores_nuevos")]
pub async fn nuevos_trabajadores(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::nuevos_trabajadores(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/rangos_edad")]
pub async fn rangos_edad(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::rangos_edad(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/rangos_antiguedad")]
pub async fn rangos_antiguedad(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::rangos_antiguedad(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/reporte_eventos")]
pub async fn reporte_eventos(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_eventos(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/comparar_mef")]
pub async fn comparar_mef(
    data: web::Data<AppState>,
    payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let (bytes_cas, bytes_otros, _) = leer_multipart_airhsp(payload).await?;
    let result = dash_service::comparar_mef(&data.db, bytes_cas, bytes_otros).await?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn leer_multipart_airhsp(
    mut payload: Multipart,
) -> Result<(Vec<u8>, Vec<u8>, String), ApiError> {
    use futures_util::StreamExt;
    let mut bytes_cas = Vec::new();
    let mut bytes_otros = Vec::new();
    let mut confirmar = String::new();
    let max_size = 15 * 1024 * 1024;
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error leyendo payload multipart: {}", e)))?
    {
        if let Some(cd) = field.content_disposition() {
            if let Some(name) = cd.get_name() {
                let is_cas = name == "file_cas";
                let is_otros = name == "file_otros";
                if is_cas || is_otros {
                    let mut file_size = 0;
                    while let Some(chunk) = field.next().await {
                        let data_chunk = chunk.map_err(|e| {
                            ApiError::InternalError(format!("Error leyendo archivo: {}", e))
                        })?;
                        file_size += data_chunk.len();
                        if file_size > max_size {
                            return Err(ApiError::BadRequest(
                                "El archivo no debe superar los 15 MB".to_string(),
                            ));
                        }
                        if is_cas {
                            bytes_cas.extend_from_slice(&data_chunk);
                        } else {
                            bytes_otros.extend_from_slice(&data_chunk);
                        }
                    }
                } else if name == "confirmar" {
                    while let Some(chunk) = field.next().await {
                        let data_chunk = chunk.map_err(|e| {
                            ApiError::InternalError(format!("Error leyendo campo confirmar: {}", e))
                        })?;
                        confirmar.push_str(&String::from_utf8_lossy(&data_chunk));
                    }
                }
            }
        }
    }
    Ok((bytes_cas, bytes_otros, confirmar))
}

#[post("/aplicar_airhsp")]
pub async fn aplicar_airhsp(
    req: HttpRequest,
    data: web::Data<AppState>,
    payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let (bytes_cas, bytes_otros, confirmar_str) = leer_multipart_airhsp(payload).await?;
    let confirmar = confirmar_str.trim().eq_ignore_ascii_case("true");

    if confirmar {
        let role = req
            .extensions()
            .get::<crate::common::jwt::Claims>()
            .map(|c| c.role.to_uppercase());

        match role.as_deref() {
            Some("ADMIN") | Some("EDITOR") => {}
            _ => {
                return Err(ApiError::Forbidden(
                    "No tiene permisos para modificar información".to_string(),
                ));
            }
        }
    }

    let result =
        dash_service::aplicar_airhsp(&data.db, &req, bytes_cas, bytes_otros, confirmar).await?;
    Ok(HttpResponse::Ok().json(result))
}


#[post("/generar_mef")]
pub async fn generar_mef(payload: web::Json<serde_json::Value>) -> Result<HttpResponse, ApiError> {
    let comparaciones = payload
        .get("comparaciones")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            ApiError::BadRequest("Falta el array 'comparaciones' en el body".to_string())
        })?;
    let buffer = dash_service::exportar_comparacion_mef(comparaciones)?;
    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"comparacion_mef.xlsx\"",
        ))
        .body(buffer))
}

#[get("/alerta_70")]
pub async fn alerta_70_anos(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::alerta_70_anos(&data.db).await?;
    Ok(HttpResponse::Ok().json(result))
}
