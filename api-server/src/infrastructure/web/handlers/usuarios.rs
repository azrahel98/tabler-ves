use super::registrar_historial;
use crate::AppState;
use crate::infrastructure::db::repositories::usuario_repo;
use crate::infrastructure::web::middleware::{
    error::{ApiError, validar},
    jwt::Claims,
};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use validator::Validate;

fn verificar_admin(req: &HttpRequest) -> Result<(), ApiError> {
    let role = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.role.clone())
        .unwrap_or_default();
    if role.to_uppercase() != "ADMIN" {
        return Err(ApiError::Unauthorized(
            "Acceso restringido a administradores".into(),
        ));
    }
    Ok(())
}

pub async fn listar_usuarios(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    let usuarios = usuario_repo::list_all(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al listar usuarios: {}", e)))?;
    Ok(HttpResponse::Ok().json(usuarios))
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

pub async fn crear_usuario(
    data: web::Data<AppState>,
    body: web::Json<CrearUsuarioBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;

    let user_id = usuario_repo::create_user(
        &data.db,
        &body.google_sub,
        &body.email,
        &body.full_name,
        body.picture_url.as_deref(),
    )
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al crear usuario: {}", e)))?;

    let _ = registrar_historial(
        &req,
        &data.db,
        "crear usuario",
        "",
        Some(serde_json::json!({
            "id": user_id,
            "google_sub": body.google_sub,
            "email": body.email,
            "full_name": body.full_name
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json("Usuario creado correctamente"))
}

#[derive(Deserialize, Serialize, Validate)]
pub struct EditarUsuarioBody {
    #[validate(range(min = 1, message = "ID inválido"))]
    pub id: i32,
    pub role: String,
    pub status: String,
}

pub async fn editar_usuario(
    data: web::Data<AppState>,
    body: web::Json<EditarUsuarioBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    validar(&body.0)?;

    let actual = usuario_repo::find_by_id(&data.db, body.id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener usuario actual: {}", e)))?;

    let user_actual = match actual {
        Some(u) => u,
        None => return Err(ApiError::BadRequest("Usuario no encontrado".into())),
    };

    let role_upper = body.role.to_uppercase();
    let status_upper = body.status.to_uppercase();

    if role_upper != "ADMIN" && role_upper != "USER" {
        return Err(ApiError::BadRequest("Rol inválido (debe ser ADMIN o USER)".into()));
    }
    if status_upper != "PENDING" && status_upper != "APPROVED" && status_upper != "REJECTED" {
        return Err(ApiError::BadRequest("Estado inválido (debe ser PENDING, APPROVED o REJECTED)".into()));
    }

    usuario_repo::update_status_and_role(&data.db, body.id, &role_upper, &status_upper)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al actualizar usuario: {}", e)))?;

    let mut diff = serde_json::Map::new();
    if user_actual.role != role_upper {
        diff.insert("role".to_string(), json!({"antes": user_actual.role, "despues": role_upper}));
    }
    if user_actual.status != status_upper {
        diff.insert("status".to_string(), json!({"antes": user_actual.status, "despues": status_upper}));
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

pub async fn eliminar_usuario(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    verificar_admin(&req)?;
    let id = path.into_inner();
    let usuario_full = usuario_repo::find_by_id(&data.db, id)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener usuario: {}", e)))?;

    let caller_id = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.id)
        .unwrap_or(0);

    if caller_id == id {
        return Err(ApiError::BadRequest(
            "No puedes eliminar tu propia cuenta".into(),
        ));
    }

    usuario_repo::delete_user(&data.db, id)
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
                    "email": u.email,
                    "full_name": u.full_name,
                    "role": u.role,
                    "status": u.status
                }
            })),
        )
        .await;
    }

    Ok(HttpResponse::Ok().json("Usuario eliminado"))
}

