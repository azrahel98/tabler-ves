use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(u16, String),
    Unauthorized(u16, String),
    InternalError(u16, String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(_, msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(_, msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::InternalError(_, msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(code, msg) => HttpResponse::BadRequest().json(ErrorResponse {
                code: *code,
                error: msg.clone(),
            }),
            ApiError::Unauthorized(code, msg) => HttpResponse::Unauthorized().json(ErrorResponse {
                code: *code,
                error: msg.clone(),
            }),
            ApiError::InternalError(code, msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: *code,
                    error: msg.clone(),
                })
            }
        }
    }
}
