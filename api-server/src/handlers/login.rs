use crate::{
    AppState,
    middleware::{
        error::{ApiError, validar},
        jwt::generate_token,
    },
    models::login::Usuario,
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};

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

    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");

    let user = sqlx::query_as!(
        Usuario,
        r#"
        SELECT
            id,
            nombre,
            nickname,
            CAST(aes_decrypt(pass,?) AS CHAR) as pass,
            nivel
        FROM usuario
        WHERE nickname = ?
        "#,
        key,
        login.username
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    let user = match user {
        Some(u) => {
            if u.pass.as_deref() != Some(login.password.as_str()) {
                return Err(ApiError::Unauthorized("Contraseña incorrecta".into()));
            }
            u
        }
        None => return Err(ApiError::Unauthorized("El usuario no existe".into())),
    };

    let token = generate_token(user.id, user.nivel, user.nombre);

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
) -> Result<impl Responder, ApiError> {
    validar(&pass.0)?;
    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");

    let db_pass = sqlx::query_scalar!(
        r#"
        SELECT CAST(aes_decrypt(pass, ?) AS CHAR)
        FROM usuario
        WHERE id = ?
        "#,
        key,
        pass.id,
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?
    .unwrap();

    let db_pass = match db_pass {
        Some(p) => p,
        None => {
            return Err(ApiError::Unauthorized(
                "Usuario no encontrado o sin contraseña".into(),
            ));
        }
    };

    if db_pass != pass.oldpass {
        return Err(ApiError::Unauthorized(
            "La contraseña no es correcta".into(),
        ));
    }

    sqlx::query!(
        r#"
        UPDATE usuario SET pass = aes_encrypt(?, ?) WHERE id = ?
        "#,
        pass.newpass,
        key,
        pass.id,
    )
    .execute(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("No se pudo actualizar la contraseña".into())
    })?;

    Ok(HttpResponse::Ok().json("Contraseña cambiada con éxito"))
}
