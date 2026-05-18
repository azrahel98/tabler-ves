use crate::middleware::error::ApiError;
use crate::models::login::Usuario;
use crate::repositories::usuario_repo;
use sqlx::MySqlPool;
pub enum LoginResult {
    Success(Usuario),
    InvalidPassword,
    UserNotFound,
}
pub async fn authenticate(
    db: &MySqlPool,
    nickname: &str,
    password: &str,
) -> Result<LoginResult, ApiError> {
    let user = usuario_repo::find_by_nickname(db, nickname)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Database consulta malformada".into())
        })?;
    match user {
        Some(u) => {
            if u.pass.as_deref() != Some(password) {
                Ok(LoginResult::InvalidPassword)
            } else {
                Ok(LoginResult::Success(u))
            }
        }
        None => Ok(LoginResult::UserNotFound),
    }
}
pub async fn change_password(
    db: &MySqlPool,
    id: i32,
    oldpass: &str,
    newpass: &str,
) -> Result<(), ApiError> {
    let db_pass = usuario_repo::get_password_by_id(db, id)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Database consulta malformada".into())
        })?;
    let db_pass = match db_pass {
        Some(p) => p,
        None => {
            return Err(ApiError::Unauthorized(
                "Usuario no encontrado o sin contraseña".into(),
            ));
        }
    };
    if db_pass != oldpass {
        return Err(ApiError::Unauthorized(
            "La contraseña no es correcta".into(),
        ));
    }
    usuario_repo::update_password(db, id, newpass)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("No se pudo actualizar la contraseña".into())
        })?;
    Ok(())
}
