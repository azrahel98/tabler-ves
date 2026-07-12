use crate::AppState;
use crate::infrastructure::web::middleware::error::{ApiError, validar};
use crate::infrastructure::web::models::personal::GradoAcademico;
use crate::application::usecases::gradoaca_service;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use validator::Validate;
use super::registrar_historial;
#[derive(Deserialize, Validate)]
pub struct GradoDniBody {
    #[validate(custom(function = "crate::infrastructure::web::models::personal::es_dni_valido"))]
    pub dni: String,
}

pub async fn grado_por_dni(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    let gradoac = gradoaca_service::get_por_dni(&data.db, &dni).await?;
    Ok(HttpResponse::Ok().json(gradoac))
}

pub async fn upsert_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let doc_entity = doc.0.clone().into();
    let (diff_value, accion, rows_affected) =
        gradoaca_service::upsert_gradoacademico(&data.db, &doc_entity).await?;
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
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();
    let (diff_value, accion, dni) = gradoaca_service::eliminar_gradoa(&data.db, id).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Grado académico eliminado"))
}
