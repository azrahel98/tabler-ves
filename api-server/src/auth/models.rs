use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Usuario {
    pub id: i32,
    pub google_sub: String,
    pub email: String,
    pub full_name: String,
    pub picture_url: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "google_sub es requerido"))]
    pub google_sub: String,
    #[validate(email(message = "Correo electrónico inválido"))]
    pub email: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "google_sub es requerido"))]
    pub google_sub: String,
    #[validate(email(message = "Correo electrónico inválido"))]
    pub email: String,
    #[validate(length(min = 1, message = "El nombre completo es requerido"))]
    pub full_name: String,
    pub picture_url: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CrearUsuarioBody {
    #[validate(length(min = 1, message = "google_sub es requerido"))]
    pub google_sub: String,
    #[validate(email(message = "Correo electrónico inválido"))]
    pub email: String,
    #[validate(length(min = 1, message = "El nombre completo es requerido"))]
    pub full_name: String,
    pub picture_url: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct EditarUsuarioBody {
    #[validate(range(min = 1, message = "ID inválido"))]
    pub id: i32,
    pub role: String,
    pub status: String,
}

pub enum LoginResult {
    Success(Usuario),
    UserNotFound,
    PendingApproval,
    AccountRejected,
}
