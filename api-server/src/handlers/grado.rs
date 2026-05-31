use crate::{
    AppState,
    middleware::error::{ApiError, validar},
    models::personal::GradoAcademico,
    services::gradoaca_service,
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use validator::Validate;
use super::registrar_historial;

#[derive(Deserialize, Validate)]
pub struct GradoDniBody {
    #[validate(custom(function = "crate::models::personal::es_dni_valido"))]
    pub dni: String,
}

pub async fn grado_por_dni(
    data: web::Data<AppState>,
    body: web::Json<GradoDniBody>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let gradoac = gradoaca_service::get_por_dni(&data.db, &body.dni).await?;
    Ok(HttpResponse::Ok().json(gradoac))
}

pub async fn upsert_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let (diff_value, accion, rows_affected) =
        gradoaca_service::upsert_gradoacademico(&data.db, &doc.0).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &doc.dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json(format!(
        "Operación exitosa. Filas afectadas: {}",
        rows_affected
    )))
}

#[derive(Deserialize, Validate)]
pub struct EliminarGradoBody {
    #[validate(range(min = 1, message = "ID de grado inválido"))]
    pub id: i32,
}

pub async fn eliminar_gradoa(
    data: web::Data<AppState>,
    body: web::Json<EliminarGradoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let (diff_value, accion, dni) = gradoaca_service::eliminar_gradoa(&data.db, body.id).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Grado académico eliminado"))
}
