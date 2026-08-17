use crate::{
    AppState,
    middleware::{
        error::{ApiError, validar},
        jwt::generate_token,
    },
    services::login::{self, LoginResult},
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use super::registrar_historial;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "El usuario es requerido"))]
    username: String,
    #[validate(length(min = 1, message = "La contraseña es requerida"))]
    password: String,
}
pub async fn login(
    data: web::Data<AppState>,
    login: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    validar(&login.0)?;
    let auth_result =
        login::authenticate(&data.db, &login.username, &login.password).await?;
    let user = match auth_result {
        LoginResult::Success(u) => u,
        LoginResult::InvalidPassword => {
            return Err(ApiError::Unauthorized("Contraseña incorrecta".into()));
        }
        LoginResult::UserNotFound => {
            return Err(ApiError::Unauthorized("El usuario no existe".into()));
        }
    };
    let token = generate_token(user.id, user.nivel, user.nombre.clone());
    let json_response = serde_json::json!({
        "token": token
    });
    Ok(HttpResponse::Ok().json(json_response))
}
#[derive(Deserialize, Validate)]
pub struct ChangePass {
    #[validate(range(min = 1, message = "ID de usuario inválido"))]
    pub id: i32,
    #[validate(length(min = 1, message = "La contraseña actual es requerida"))]
    pub oldpass: String,
    #[validate(length(
        min = 4,
        message = "La nueva contraseña debe tener al menos 4 caracteres"
    ))]
    pub newpass: String,
}
pub async fn change_pass(
    data: web::Data<AppState>,
    pass: web::Json<ChangePass>,
    req: actix_web::HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&pass.0)?;
    login::change_password(&data.db, pass.id, &pass.oldpass, &pass.newpass).await?;
    let _ = registrar_historial(&req, &data.db, "cambio de password propio", "", None).await;
    Ok(HttpResponse::Ok().json("Contraseña cambiada con éxito"))
}
