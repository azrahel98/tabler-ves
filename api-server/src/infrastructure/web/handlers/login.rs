use crate::AppState;
use crate::application::usecases::login::{self, LoginResult};
use crate::infrastructure::web::middleware::{
    error::{ApiError, validar},
    jwt::generate_token,
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "google_sub es requerido"))]
    pub google_sub: String,
    #[validate(email(message = "Correo electrónico inválido"))]
    pub email: String,
}

pub async fn login(
    data: web::Data<AppState>,
    login_req: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    validar(&login_req.0)?;
    let auth_result = login::authenticate_google(&data.db, &login_req.google_sub, &login_req.email).await?;
    let user = match auth_result {
        LoginResult::Success(u) => u,
        LoginResult::UserNotFound => {
            return Err(ApiError::Unauthorized("Usuario no encontrado. Por favor regístrese.".into()));
        }
        LoginResult::PendingApproval => {
            return Err(ApiError::Unauthorized("Su cuenta está pendiente de aprobación por un administrador.".into()));
        }
        LoginResult::AccountRejected => {
            return Err(ApiError::Unauthorized("Su solicitud de acceso ha sido rechazada.".into()));
        }
    };

    let token = generate_token(
        user.id,
        user.role.clone(),
        user.email.clone(),
        user.full_name.clone(),
        user.picture_url.clone(),
    );

    let json_response = serde_json::json!({
        "token": token,
        "user": user
    });
    Ok(HttpResponse::Ok().json(json_response))
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

pub async fn register(
    data: web::Data<AppState>,
    reg_req: web::Json<RegisterRequest>,
) -> Result<impl Responder, ApiError> {
    validar(&reg_req.0)?;
    let user_id = login::register_google(
        &data.db,
        &reg_req.google_sub,
        &reg_req.email,
        &reg_req.full_name,
        reg_req.picture_url.as_deref(),
    )
    .await?;

    let json_response = serde_json::json!({
        "id": user_id,
        "message": "Registro completado con éxito. Tu cuenta está pendiente de aprobación por un administrador."
    });
    Ok(HttpResponse::Created().json(json_response))
}

