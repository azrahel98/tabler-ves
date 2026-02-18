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
    #[allow(dead_code)]
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
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
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse {
                code: 404,
                error: msg.clone(),
            }),
            ApiError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    error: msg.clone(),
                })
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::InternalError(format!("Error en la base de datos: {}", error))
    }
}
