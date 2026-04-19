use super::registrar_historial;
use crate::{
    AppState,
    middleware::{
        error::{ApiError, validar},
        jwt::Claims,
    },
};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use validator::Validate;

fn verificar_admin(req: &HttpRequest) -> Result<(), ApiError> {
    let lvl = req
        .extensions()
        .get::<Claims>()
        .and_then(|c| c.lvl)
        .unwrap_or(0);
    if lvl != 1 {
        return Err(ApiError::Unauthorized(
            "Acceso restringido a administradores".into(),
        ));
    }
    Ok(())
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UsuarioListado {
    pub id: i32,
    pub nombre: String,
    pub nickname: String,
    pub nivel: i32,
}

pub async fn listar_usuarios(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    let usuarios = sqlx::query_as!(
        UsuarioListado,
        r#"SELECT id, nombre, nickname, nivel FROM usuario ORDER BY nombre"#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al listar usuarios: {}", e)))?;
    Ok(HttpResponse::Ok().json(usuarios))
}

#[derive(Deserialize, Validate)]
pub struct CrearUsuarioBody {
    #[validate(length(min = 2, message = "El nombre es requerido"))]
    pub nombre: String,
    #[validate(length(min = 3, message = "El nickname debe tener al menos 3 caracteres"))]
    pub nickname: String,
    #[validate(length(min = 4, message = "La contraseña debe tener al menos 4 caracteres"))]
    pub pass: String,
    #[validate(range(min = 1, max = 2, message = "Nivel inválido (1=admin, 2=usuario)"))]
    pub nivel: i32,
}

pub async fn crear_usuario(
    data: web::Data<AppState>,
    body: web::Json<CrearUsuarioBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;
    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");
    sqlx::query(
        "INSERT INTO usuario (nombre, nickname, pass, nivel) VALUES (?, ?, aes_encrypt(?, ?), ?)",
    )
    .bind(&body.nombre)
    .bind(&body.nickname)
    .bind(&body.pass)
    .bind(key)
    .bind(body.nivel)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al crear usuario: {}", e)))?;

    let _ = registrar_historial(
        &req,
        &data.db,
        "crear usuario",
        "",
        Some(serde_json::json!({
            "nombre": body.nombre,
            "nickname": body.nickname,
            "nivel": body.nivel
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json("Usuario creado correctamente"))
}

#[derive(Deserialize, Serialize, Validate)]
pub struct EditarUsuarioBody {
    #[validate(range(min = 1, message = "ID inválido"))]
    pub id: i32,
    #[validate(length(min = 2, message = "El nombre es requerido"))]
    pub nombre: String,
    #[validate(length(min = 3, message = "Nickname inválido"))]
    pub nickname: String,
    #[validate(range(min = 1, max = 2, message = "Nivel inválido"))]
    pub nivel: i32,
}

pub async fn editar_usuario(
    data: web::Data<AppState>,
    body: web::Json<EditarUsuarioBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;

    // 1. Obtener estado actual
    let actual = sqlx::query!(
        "SELECT nombre, nickname, nivel FROM usuario WHERE id = ?",
        body.id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al obtener usuario actual: {}", e)))?;

    sqlx::query("UPDATE usuario SET nombre = ?, nickname = ?, nivel = ? WHERE id = ?")
        .bind(&body.nombre)
        .bind(&body.nickname)
        .bind(body.nivel)
        .bind(body.id)
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al editar usuario: {}", e)))?;

    // 2. Calcular diferencias
    let mut diff = serde_json::Map::new();
    if actual.nombre != body.nombre {
        diff.insert("nombre".to_string(), json!({"antes": actual.nombre, "despues": body.nombre}));
    }
    if actual.nickname != body.nickname {
        diff.insert("nickname".to_string(), json!({"antes": actual.nickname, "despues": body.nickname}));
    }
    if actual.nivel != body.nivel {
        diff.insert("nivel".to_string(), json!({"antes": actual.nivel, "despues": body.nivel}));
    }

    if !diff.is_empty() {
        let _ = registrar_historial(
            &req,
            &data.db,
            "editar usuario",
            "",
            Some(Value::Object(diff)),
        )
        .await;
    }

    Ok(HttpResponse::Ok().json("Usuario actualizado correctamente"))
}

#[derive(Deserialize, Validate)]
pub struct EliminarUsuarioBody {
    #[validate(range(min = 1, message = "ID inválido"))]
    pub id: i32,
}

pub async fn eliminar_usuario(
    data: web::Data<AppState>,
    body: web::Json<EliminarUsuarioBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;

    // Capturar foto antes de borrar
    let usuario_full = sqlx::query!(
        "SELECT id, nombre, nickname, nivel FROM usuario WHERE id = ?",
        body.id
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al capturar usuario: {}", e)))?;

    let caller_id = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.id)
        .unwrap_or(0);
    if caller_id == body.id {
        return Err(ApiError::BadRequest(
            "No puedes eliminar tu propia cuenta".into(),
        ));
    }

    sqlx::query("DELETE FROM usuario WHERE id = ?")
        .bind(body.id)
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar usuario: {}", e)))?;

    if let Some(u) = usuario_full {
        let _ = registrar_historial(
            &req,
            &data.db,
            "eliminar usuario",
            "",
            Some(serde_json::json!({ 
                "objeto_eliminado": {
                    "id": u.id,
                    "nombre": u.nombre,
                    "nickname": u.nickname,
                    "nivel": u.nivel
                }
            })),
        )
        .await;
    }

    Ok(HttpResponse::Ok().json("Usuario eliminado"))
}

#[derive(Deserialize, Validate)]
pub struct ResetPassBody {
    #[validate(range(min = 1, message = "ID inválido"))]
    pub id: i32,
    #[validate(length(min = 4, message = "La contraseña debe tener al menos 4 caracteres"))]
    pub nueva_pass: String,
}

pub async fn reset_pass_usuario(
    data: web::Data<AppState>,
    body: web::Json<ResetPassBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;
    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");
    sqlx::query("UPDATE usuario SET pass = aes_encrypt(?, ?) WHERE id = ?")
        .bind(&body.nueva_pass)
        .bind(key)
        .bind(body.id)
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al resetear contraseña: {}", e)))?;

    let _ = registrar_historial(
        &req,
        &data.db,
        "reset password usuario",
        "",
        Some(serde_json::json!({ "id_usuario_reset": body.id })),
    )
    .await;

    Ok(HttpResponse::Ok().json("Contraseña restablecida correctamente"))
}
