use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: 400,
                error: msg.clone(),
            }),
            ApiError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                code: 401,
                error: msg.clone(),
            }),
            ApiError::Forbidden(msg) => HttpResponse::Forbidden().json(ErrorResponse {
                code: 403,
                error: msg.clone(),
            }),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                code: 404,
                error: msg.clone(),
            }),
            ApiError::Conflict(msg) => HttpResponse::Conflict().json(ErrorResponse {
                code: 409,
                error: msg.clone(),
            }),
            ApiError::InternalError(msg) => {
                log::error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    error: "Error interno del servidor".to_string(),
                })
            }
        }
    }
}

pub fn db_error(contexto: &str, error: sqlx::Error) -> ApiError {
    match &error {
        sqlx::Error::RowNotFound => {
            ApiError::NotFound(format!("No se encontró {}.", contexto))
        }
        _ => {
            if let Some(db_err) = error.as_database_error() {
                if let Some(mysql_err) = db_err.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>() {
                    match mysql_err.number() {
                        1062 => {
                            let msg = mysql_err.message();
                            let mensaje = if msg.contains("uk_documento_emision") {
                                "Ya existe un documento registrado con el mismo tipo, número, año y área emisora.".to_string()
                            } else if msg.contains("uk_tipo_doc_nombre") {
                                "Ya existe un tipo de documento con ese nombre.".to_string()
                            } else if msg.contains("fk_vst_unique") {
                                "El trabajador ya está afiliado a ese sindicato.".to_string()
                            } else if msg.contains("google_sub") || msg.contains("email") {
                                "Ya existe un usuario registrado con ese correo.".to_string()
                            } else if msg.contains("numero_cuenta") || msg.contains("cci_num") {
                                "Ya existe una cuenta bancaria registrada con ese número.".to_string()
                            } else if msg.contains("uq_convenio_nombre") {
                                "Ya existe un convenio con ese nombre, monto y régimen.".to_string()
                            } else if msg.contains("PRIMARY") {
                                format!("Ya existe un registro con ese identificador ({}).", contexto)
                            } else {
                                format!("Ya existe un registro con esos mismos datos ({}).", contexto)
                            };
                            ApiError::Conflict(mensaje)
                        }
                        1451 => ApiError::Conflict(format!(
                            "No se puede completar la operación: existen registros que dependen de {}.",
                            contexto
                        )),
                        1452 => ApiError::BadRequest(format!(
                            "Referencia inválida en {}: el registro relacionado no existe.",
                            contexto
                        )),
                        1048 => ApiError::BadRequest(format!(
                            "Falta un dato obligatorio en {}.",
                            contexto
                        )),
                        1406 => ApiError::BadRequest(format!(
                            "Un texto excede el largo permitido en {}.",
                            contexto
                        )),
                        1264 => ApiError::BadRequest(format!(
                            "Un valor numérico está fuera de rango en {}.",
                            contexto
                        )),
                        1292 | 1366 => ApiError::BadRequest(format!(
                            "Formato de valor inválido en {}.",
                            contexto
                        )),
                        _ => ApiError::InternalError(format!("{}: {}", contexto, error)),
                    }
                } else {
                    ApiError::InternalError(format!("{}: {}", contexto, error))
                }
            } else {
                ApiError::InternalError(format!("{}: {}", contexto, error))
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        db_error("la operación en base de datos", error)
    }
}

pub fn validar<T: validator::Validate>(dato: &T) -> Result<(), ApiError> {
    dato.validate().map_err(|errores| {
        let mensaje = errores
            .field_errors()
            .iter()
            .map(|(campo, lista)| {
                let detalle = lista
                    .iter()
                    .filter_map(|e| e.message.as_ref())
                    .map(|m| m.as_ref())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}: {}", campo, detalle)
            })
            .collect::<Vec<_>>()
            .join(" | ");
        ApiError::BadRequest(mensaje)
    })
}
