use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, web};
use futures_util::TryStreamExt;
use sqlx::MySqlPool;

use crate::{
    application::usecases::dash_service, infrastructure::web::middleware::error::ApiError,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistorialQuery {
    dni: String,
    key: String,
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

#[derive(Deserialize)]
pub struct Alerta70Query {
    pub edad_min: Option<i32>,
}


#[get("/cumpleanos")]
pub async fn cumpleanos(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let list = dash_service::cumpleaños(&pool).await?;
    Ok(HttpResponse::Ok().json(list))
}

#[get("/resumen")]
pub async fn info(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::info(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/areareport")]
pub async fn personal_area_report(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_area_report(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/renunciasano")]
pub async fn renuncias_ano(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::renuncias_ano(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/bancosreport")]
pub async fn bancos_report(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::bancos_report(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos")]
pub async fn reporte_personal_activo(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_personal_activo(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/area")]
pub async fn personal_activo_area(
    pool: web::Data<MySqlPool>,
    query: web::Query<AreaQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_activo_area(&pool, query.area_id).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/sindicato")]
pub async fn personal_activo_sindicato(
    pool: web::Data<MySqlPool>,
    query: web::Query<SindicatoQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_activo_sindicato(
        &pool,
        query.sindicato_id,
        query.sindicato.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/activos/regimen")]
pub async fn personal_activo_regimen(
    pool: web::Data<MySqlPool>,
    query: web::Query<RegimenQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::personal_activo_regimen(
        &pool,
        query.regimen_id,
        query.regimen.as_deref(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(result))
}



#[get("/historial")]
pub async fn reporte_historial(
    pool: web::Data<MySqlPool>,
    query: web::Query<HistorialQuery>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_historial(&pool, &query.dni, &query.key).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/organigrama")]
pub async fn organigrama(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::organigrama(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/report-renuncia")]
pub async fn report_renuncias(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::report_renuncias(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/documentos")]
pub async fn reporte_documentos(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_documentos(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/exportar_excel")]
pub async fn exportar_excel(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let buffer = dash_service::exportar_excel(&pool).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .append_header((
            "Content-Disposition",
            "attachment; filename=\"reporte_airhsp.xlsx\"",
        ))
        .body(buffer))
}

#[get("/activos/distrito")]
pub async fn activos_por_distrito(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::activos_por_distrito(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/trabajadores_nuevos")]
pub async fn nuevos_trabajadores(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    println!("esto aquiiii!");
    let result = dash_service::nuevos_trabajadores(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/rangos_edad")]
pub async fn rangos_edad(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::rangos_edad(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/rangos_antiguedad")]
pub async fn rangos_antiguedad(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::rangos_antiguedad(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/reporte_eventos")]
pub async fn reporte_eventos(pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let result = dash_service::reporte_eventos(&pool).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/comparar_mef")]
pub async fn comparar_mef(
    pool: web::Data<MySqlPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    use futures_util::StreamExt;
    let mut bytes_cas = Vec::new();
    let mut bytes_otros = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(cd) = field.content_disposition() {
            if let Some(name) = cd.get_name() {
                let is_cas = name == "file_cas";
                let is_otros = name == "file_otros";
                if is_cas || is_otros {
                    while let Some(chunk) = field.next().await {
                        let data = chunk.map_err(|e| {
                            ApiError::InternalError(format!("Error leyendo archivo: {}", e))
                        })?;
                        if is_cas {
                            bytes_cas.extend_from_slice(&data);
                        } else {
                            bytes_otros.extend_from_slice(&data);
                        }
                    }
                }
            }
        }
    }
    let result = dash_service::comparar_mef(&pool, bytes_cas, bytes_otros).await?;
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
pub async fn alerta_70_anos(
    pool: web::Data<MySqlPool>,
    query: web::Query<Alerta70Query>,
) -> Result<HttpResponse, ApiError> {
    let result = dash_service::alerta_70_anos(&pool, query.edad_min).await?;
    Ok(HttpResponse::Ok().json(result))
}

