use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, web};
use futures_util::stream::StreamExt as _;
use std::fs as std_fs;
use std::io::Write;
use uuid::Uuid;

use crate::{AppState, UPLOAD_DIR, middleware::error::ApiError};

pub async fn upload_avatar(
    data: web::Data<AppState>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return Ok(HttpResponse::BadRequest().body("Error en la carga")),
        };

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => return Ok(HttpResponse::BadRequest().body("Falta Content-Disposition")),
        };

        if let Some(filename) = content_disposition.get_filename() {
            let extension = std::path::Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("bin");

            let new_filename = format!("{}.{}", Uuid::new_v4(), extension);
            let filepath = format!("{}/{}", UPLOAD_DIR, new_filename);
            let public_url = format!("/fotos/{}", new_filename);

            // 1. GUARDADO DEL NUEVO ARCHIVO (Tu lógica actual)
            let filepath_clone = filepath.clone();
            let f = web::block(move || std_fs::File::create(filepath_clone))
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| actix_web::error::ErrorBadRequest(e))?;
                let mut f = f
                    .try_clone()
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                web::block(move || f.write_all(&data))
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            }

            // --- NUEVO: 2. OBTENER LA RUTA ANTIGUA ANTES DE SOBRESCRIBIR ---
            // Consultamos qué avatar tiene actualmente el usuario
            let old_avatar_url: Option<String> =
                sqlx::query_scalar("SELECT avatar FROM Persona WHERE dni = ?")
                    .bind(&user_id)
                    .fetch_optional(&data.db)
                    .await
                    .unwrap_or(None); // Si falla la consulta, asumimos que no hay foto vieja

            // 3. ACTUALIZACIÓN SQL (Tu lógica actual)
            let db_result = sqlx::query(r#"UPDATE Persona SET avatar = ? WHERE dni = ?"#)
                .bind(&public_url)
                .bind(&user_id)
                .execute(&data.db)
                .await;

            match db_result {
                Ok(_) => {
                    // --- NUEVO: 4. BORRADO FÍSICO DEL ARCHIVO VIEJO ---
                    if let Some(old_url) = old_avatar_url {
                        // old_url viene como "/fotos/abc.png"
                        // Necesitamos convertirlo a ruta de sistema: "./uploads/abc.png"

                        // Quitamos el prefijo "/fotos/" y dejamos solo el nombre
                        let old_filename = old_url.replace("/fotos/", "");

                        // Construimos la ruta física vieja
                        let old_filepath = format!("{}/{}", UPLOAD_DIR, old_filename);

                        // Intentamos borrar. Usamos un bloque para no detener el programa si falla
                        // (ej. si el archivo ya había sido borrado manualmente)
                        if let Err(e) = std_fs::remove_file(&old_filepath) {
                            println!(
                                "Advertencia: No se pudo borrar el avatar antiguo ({}): {}",
                                old_filepath, e
                            );
                        } else {
                            println!("Avatar antiguo borrado: {}", old_filepath);
                        }
                    }

                    return Ok(HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": "Avatar actualizado correctamente",
                        "url": public_url
                    })));
                }
                Err(e) => {
                    // Si falla la BD, borramos la imagen NUEVA que acabamos de subir para no dejar basura
                    let _ = std_fs::remove_file(filepath);
                    return Err(ApiError::InternalError(3, format!("Error BD: {}", e)).into());
                }
            }
        }
    }

    Ok(HttpResponse::BadRequest().body("No se procesó ningún archivo"))
}
