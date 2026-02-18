use crate::{
    AppState,
    key::{self, key::DB_KEY},
    middleware::{error::ApiError, jwt::generate_token},
    models::login::Usuario,
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.username.trim().is_empty() {
            return Err(ApiError::BadRequest("username is required".into()));
        }
        if self.password.trim().is_empty() {
            return Err(ApiError::BadRequest("password is required".into()));
        }

        Ok(())
    }
}

pub async fn login(
    data: web::Data<AppState>,
    login: web::Json<LoginRequest>,
) -> Result<impl Responder, ApiError> {
    login.validate()?;

    let key = key::key::DB_KEY;

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

#[derive(Deserialize)]
pub struct ChangePass {
    pub id: i32,
    pub oldpass: String,
    pub newpass: String,
}

pub async fn change_pass(
    data: web::Data<AppState>,
    pass: web::Json<ChangePass>,
) -> Result<impl Responder, ApiError> {
    let key = DB_KEY;

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
