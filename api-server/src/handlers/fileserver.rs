use crate::{
    AppState,
    middleware::{error::ApiError, jwt::Claims},
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use futures_util::TryStreamExt;
use serde::Deserialize;
use serde_json::json;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub async fn upload_file(
    data: web::Data<AppState>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut documento_id: Option<i32> = None;
    let mut dni_asociado: Option<String> = None;

    let claims = req.extensions().get::<Claims>().cloned();
    let usuario_subida = claims
        .map(|c| c.nombre)
        .unwrap_or_else(|| "Desconocido".to_string());

    let mut saved_files = Vec::new();

    let upload_dir_str = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let upload_dir = std::path::PathBuf::from(upload_dir_str);
    if !upload_dir.exists() {
        std::fs::create_dir_all(&upload_dir).map_err(|e| {
            ApiError::InternalError(format!("Error creating upload directory: {}", e))
        })?;
    }

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error reading multipart payload: {}", e)))?
    {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition
            .and_then(|cd| cd.get_name())
            .unwrap_or("");

        if field_name == "documento_id" {
            let mut val = Vec::new();
            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(|e| ApiError::InternalError(format!("Error reading chunk: {}", e)))?
            {
                val.extend_from_slice(&chunk);
            }
            if let Ok(s) = String::from_utf8(val) {
                documento_id = s.parse().ok();
            }
        } else if field_name == "dni_asociado" {
            let mut val = Vec::new();
            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(|e| ApiError::InternalError(format!("Error reading chunk: {}", e)))?
            {
                val.extend_from_slice(&chunk);
            }
            if let Ok(s) = String::from_utf8(val) {
                dni_asociado = Some(s);
            }
        } else if field_name == "file" || field_name == "archivo" {
            let original_name = content_disposition
                .and_then(|cd| cd.get_filename())
                .unwrap_or("unknown")
                .to_string();

            let extension = std::path::Path::new(&original_name)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("bin")
                .to_string();

            if extension.to_lowercase() != "pdf" {
                return Err(ApiError::InternalError(
                    "Solo se permiten archivos PDF".to_string(),
                ));
            }

            let file_hash = Uuid::new_v4().to_string();
            let mut file_path = upload_dir.clone();
            file_path.push(format!("{}.{}", file_hash, extension));

            let mut f = std::fs::File::create(&file_path).map_err(|e| {
                ApiError::InternalError(format!("Error creating file on disk: {}", e))
            })?;

            let mut current_size: usize = 0;
            let max_file_size: usize = 10 * 1024 * 1024; // 10 MB

            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(|e| ApiError::InternalError(format!("Error reading file chunk: {}", e)))?
            {
                current_size += chunk.len();
                if current_size > max_file_size {
                    // Intenta eliminar el archivo parcial si se excede el tamaño
                    let _ = std::fs::remove_file(&file_path);
                    return Err(ApiError::InternalError(
                        "El archivo no debe pesar más de 10 MB".to_string(),
                    ));
                }

                f = web::block(move || f.write_all(&chunk).map(|_| f))
                    .await
                    .map_err(|e| ApiError::InternalError(format!("Blocking thread error: {}", e)))?
                    .map_err(|e| {
                        ApiError::InternalError(format!("Error writing to file: {}", e))
                    })?;
            }

            saved_files.push((original_name, file_hash, extension));
        }
    }

    let dni_asoc = dni_asociado.ok_or_else(|| {
        ApiError::InternalError("El campo dni_asociado es obligatorio.".to_string())
    })?;

    let mut results = Vec::new();

    for (orig_name, hash, ext) in saved_files {
        let insert_result = sqlx::query!(
            r#"
            INSERT INTO fileserver (documento_id, dni_asociado, original_name, file_hash, extension, usuario_subida)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            documento_id,
            dni_asoc,
            orig_name,
            hash,
            ext,
            usuario_subida
        )
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

        results.push(json!({
            "id": insert_result.last_insert_id(),
            "original_name": orig_name,
            "file_hash": hash,
            "extension": ext
        }));
    }
    Ok(HttpResponse::Ok().json(results))
}

#[derive(Deserialize)]
pub struct ListarArchivosDniInput {
    pub dni: String,
}

pub async fn listar_archivos_dni(
    state: web::Data<AppState>,
    input: web::Json<ListarArchivosDniInput>,
) -> Result<impl Responder, ApiError> {
    let records = sqlx::query!(
        r#"
        SELECT 
            id, 
            documento_id, 
            dni_asociado, 
            original_name, 
            CAST(file_hash AS CHAR) AS file_hash, 
            extension, 
            usuario_subida, 
            CAST(fecha_subida AS CHAR) AS fecha_subida
        FROM fileserver
        WHERE dni_asociado = ?
        ORDER BY fecha_subida DESC
        "#,
        input.dni
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    let mut results = Vec::new();
    for rec in records {
        results.push(json!({
            "id": rec.id,
            "documento_id": rec.documento_id,
            "dni_asociado": rec.dni_asociado,
            "original_name": rec.original_name,
            "file_hash": rec.file_hash,
            "extension": rec.extension,
            "usuario_subida": rec.usuario_subida,
            "fecha_subida": rec.fecha_subida,
        }));
    }

    Ok(HttpResponse::Ok().json(results))
}

#[derive(Deserialize)]
pub struct EliminarArchivoInput {
    pub id: i32,
}

pub async fn eliminar_archivo(
    state: web::Data<AppState>,
    input: web::Json<EliminarArchivoInput>,
) -> Result<impl Responder, ApiError> {
    let file_record = sqlx::query!(
        r#"
        SELECT CAST(file_hash AS CHAR) AS file_hash, extension FROM fileserver WHERE id = ?
        "#,
        input.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    if let Some(record) = file_record {
        let upload_dir_str =
            std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
        let mut file_path = std::path::PathBuf::from(upload_dir_str);

        // record.file_hash can be an Option<String> because CAST can return NULL if value is NULL,
        // but since file_hash is NOT NULL, we can safely unwrap or map.
        let file_hash_str = record.file_hash.unwrap_or_else(|| "unknown".to_string());
        let extension = record.extension.unwrap_or_else(|| "pdf".to_string());

        file_path.push(format!("{}.{}", file_hash_str, extension));

        if file_path.exists() {
            let _ = std::fs::remove_file(&file_path);
        }

        sqlx::query!(
            r#"
            DELETE FROM fileserver WHERE id = ?
            "#,
            input.id
        )
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

        Ok(HttpResponse::Ok().json("Archivo eliminado correctamente"))
    } else {
        Err(ApiError::InternalError("Archivo no encontrado".to_string()))
    }
}

pub async fn ver_archivo(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let hash = path.into_inner();

    println!("{}    ", hash);

    let file_record = sqlx::query!(
        r#"
        SELECT original_name, extension FROM fileserver WHERE CAST(file_hash AS CHAR) = ?
        "#,
        hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    if let Some(record) = file_record {
        let upload_dir_str =
            std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
        let mut file_path = PathBuf::from(upload_dir_str);

        let extension = record
            .extension
            .clone()
            .unwrap_or_else(|| "pdf".to_string());
        file_path.push(format!("{}.{}", hash, extension));

        if file_path.exists() {
            let named_file = NamedFile::open(file_path)
                .map_err(|e| ApiError::InternalError(format!("Error opening file: {}", e)))?;

            use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};

            let original_name = record.original_name;

            let cd = ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::Filename(original_name)],
            };

            Ok(named_file.set_content_disposition(cd))
        } else {
            Err(ApiError::InternalError(
                "Archivo no encontrado en el disco".to_string(),
            ))
        }
    } else {
        Err(ApiError::InternalError(
            "Archivo no encontrado en la base de datos".to_string(),
        ))
    }
}
