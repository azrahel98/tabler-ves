use crate::{
    AppState,
    middleware::{error::ApiError, jwt::Claims},
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{Either, HttpMessage, HttpRequest, HttpResponse, Responder, web};
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
            let max_file_size: usize = 10 * 1024 * 1024; 

            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(|e| ApiError::InternalError(format!("Error reading file chunk: {}", e)))?
            {
                current_size += chunk.len();
                if current_size > max_file_size {
                    
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
            external_url,
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
            "external_url": rec.external_url,
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
        SELECT CAST(file_hash AS CHAR) AS file_hash, extension, external_url
        FROM fileserver WHERE id = ?
        "#,
        input.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    if let Some(record) = file_record {
        if record.external_url.is_none() {
            let upload_dir_str =
                std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
            let mut file_path = std::path::PathBuf::from(upload_dir_str);

            let file_hash_str = record.file_hash.unwrap_or_else(|| "unknown".to_string());
            let extension = record.extension.unwrap_or_else(|| "pdf".to_string());

            file_path.push(format!("{}.{}", file_hash_str, extension));

            if file_path.exists() {
                let _ = std::fs::remove_file(&file_path);
            }
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

#[derive(Deserialize)]
pub struct AsignarDocumentoInput {
    pub id: i32,
    pub documento_id: Option<i32>,
}

pub async fn asignar_documento(
    state: web::Data<AppState>,
    input: web::Json<AsignarDocumentoInput>,
) -> Result<impl Responder, ApiError> {
    sqlx::query!(
        r#"
        UPDATE fileserver SET documento_id = ? WHERE id = ?
        "#,
        input.documento_id,
        input.id
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Documento asignado correctamente" })))
}

#[derive(Deserialize)]
pub struct DocumentosPorDniInput {
    pub dni: String,
}

pub async fn documentos_por_dni(
    state: web::Data<AppState>,
    input: web::Json<DocumentosPorDniInput>,
) -> Result<impl Responder, ApiError> {
    let records = sqlx::query!(
        r#"
        SELECT
  d.id,
  CONCAT(td.nombre, ' N° ', d.numero, '-', d.year, '-', td.sigla) AS descripcion_doc,
  d.fecha,
  d.descripcion
FROM
  vinculo v
  LEFT JOIN documento d   ON d.id = v.doc_ingreso_id
  LEFT JOIN tipodocumento td ON td.id = d.tipo_documento_id
WHERE
  v.dni = ?
  AND d.id IS NOT NULL and d.numero is not null

UNION ALL

SELECT
  ds.id,
  CONCAT(tds.nombre, ' N° ', ds.numero, '-', ds.year, '-', tds.sigla) AS descripcion_doc,
  ds.fecha,
  ds.descripcion
FROM
  vinculo v
  LEFT JOIN documento ds   ON ds.id = v.doc_salida_id
  LEFT JOIN tipodocumento tds ON tds.id = ds.tipo_documento_id
WHERE
  v.dni = ?
  AND ds.id IS NOT NULL and ds.numero is not null;
        "#,
        input.dni,
        input.dni,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    let mut results = Vec::new();
    for rec in records {
        results.push(json!({
            "id": rec.id,
            "sigla": rec.descripcion_doc,
            "fecha": rec.fecha,
            "descripcion": rec.descripcion,
        }));
    }

    Ok(HttpResponse::Ok().json(results))
}

pub async fn upload_batch(
    data: web::Data<AppState>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tipo_documento_id: Option<i32> = None;
    let mut numero: Option<String> = None;
    let mut year: Option<i32> = None;
    let mut fecha: Option<String> = None;
    let mut fecha_valida: Option<String> = None;
    let mut descripcion: Option<String> = None;
    let mut nombre_archivo: Option<String> = None;
    let mut dnis: Vec<String> = Vec::new();

    let claims = req.extensions().get::<Claims>().cloned();
    let usuario_subida = claims
        .map(|c| c.nombre)
        .unwrap_or_else(|| "Desconocido".to_string());

    let mut saved_file: Option<(String, String, String)> = None;

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
        let field_name = field.content_disposition()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        let mut val = Vec::new();
        
        if field_name != "file" && field_name != "archivo" {
            while let Some(chunk) = field.try_next().await.map_err(|e| ApiError::InternalError(format!("Error reading chunk: {}", e)))? {
                val.extend_from_slice(&chunk);
            }
        }

        match field_name.as_str() {
            "tipo_documento_id" => {
                if let Ok(s) = String::from_utf8(val) {
                    tipo_documento_id = s.parse().ok();
                }
            },
            "numero" => {
                numero = String::from_utf8(val).ok();
            },
            "year" => {
                if let Ok(s) = String::from_utf8(val) {
                    year = s.parse().ok();
                }
            },
            "fecha" => {
                fecha = String::from_utf8(val).ok();
            },
            "fecha_valida" => {
                fecha_valida = String::from_utf8(val).ok();
            },
            "descripcion" => {
                descripcion = String::from_utf8(val).ok();
            },
            "nombre_archivo" => {
                nombre_archivo = String::from_utf8(val).ok();
            },
            "dnis" => {
                if let Ok(s) = String::from_utf8(val) {
                    if s.starts_with('[') {
                        if let Ok(parsed_dnis) = serde_json::from_str::<Vec<String>>(&s) {
                            dnis = parsed_dnis;
                        }
                    } else {
                        dnis = s.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                    }
                }
            },
            "file" | "archivo" => {
                let original_name = field.content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("unknown")
                    .to_string();

                let extension = std::path::Path::new(&original_name)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("bin")
                    .to_string();

                if extension.to_lowercase() != "pdf" {
                    return Err(ApiError::InternalError("Solo se permiten archivos PDF".to_string()));
                }

                let file_hash = Uuid::new_v4().to_string();
                let mut file_path = upload_dir.clone();
                file_path.push(format!("{}.{}", file_hash, extension));

                let mut f = std::fs::File::create(&file_path).map_err(|e| {
                    ApiError::InternalError(format!("Error creating file on disk: {}", e))
                })?;

                let mut current_size: usize = 0;
                let max_file_size: usize = 20 * 1024 * 1024; 

                while let Some(chunk) = field.try_next().await.map_err(|e| ApiError::InternalError(format!("Error reading file chunk: {}", e)))? {
                    current_size += chunk.len();
                    if current_size > max_file_size {
                        let _ = std::fs::remove_file(&file_path);
                        return Err(ApiError::InternalError("El archivo no debe pesar más de 20 MB".to_string()));
                    }
                    f.write_all(&chunk).map_err(|e| ApiError::InternalError(format!("Error writing chunk to file: {}", e)))?;
                }
                saved_file = Some((original_name, file_hash, extension));
            },
            _ => {}
        }
    }

    let (orig_name, hash, ext) = saved_file.ok_or_else(|| ApiError::InternalError("No se recibió ningún archivo".to_string()))?;
    
    
    let mut nombre_final = nombre_archivo.unwrap_or(orig_name);
    if !nombre_final.to_lowercase().ends_with(".pdf") {
        nombre_final = format!("{}.pdf", nombre_final);
    }

    let dnis_count = dnis.len();
    if dnis_count == 0 {
        return Err(ApiError::InternalError("Debe proporcionar al menos un DNI".to_string()));
    }

    let mut tx = data.db.begin().await.map_err(|e| ApiError::InternalError(format!("Transaction error: {}", e)))?;

    
    let doc_result = sqlx::query!(
        r#"
        INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        tipo_documento_id,
        numero,
        year,
        fecha,
        fecha_valida,
        descripcion
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error creating documento: {}", e)))?;

    let new_documento_id = doc_result.last_insert_id() as i32;

    
    for dni in &dnis {
        sqlx::query!(
            r#"
            INSERT INTO fileserver (documento_id, dni_asociado, original_name, file_hash, extension, usuario_subida)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            new_documento_id,
            dni,
            nombre_final,
            hash,
            ext,
            usuario_subida
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Database error linking DNI {}: {}", dni, e)))?;
    }

    tx.commit().await.map_err(|e| ApiError::InternalError(format!("Transaction commit error: {}", e)))?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Documento registrado y vinculado correctamente a todos los trabajadores",
        "documento_id": new_documento_id,
        "file_hash": hash,
        "count": dnis_count
    })))
}

pub async fn ver_archivo(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<Either<HttpResponse, NamedFile>, ApiError> {
    let hash = path.into_inner();

    let file_record = sqlx::query!(
        r#"
        SELECT original_name, extension, external_url
        FROM fileserver
        WHERE CAST(file_hash AS CHAR) = ?
        "#,
        hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    let record = file_record.ok_or_else(|| {
        ApiError::NotFound("Archivo no encontrado en la base de datos".to_string())
    })?;

    if let Some(url) = record.external_url {
        return Ok(Either::Left(
            HttpResponse::Found()
                .append_header(("Location", url.as_str()))
                .finish(),
        ));
    }

    let upload_dir_str =
        std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let mut file_path = PathBuf::from(upload_dir_str);

    let extension = record.extension.clone().unwrap_or_else(|| "pdf".to_string());
    file_path.push(format!("{}.{}", hash, extension));

    if !file_path.exists() {
        return Err(ApiError::NotFound(
            "Archivo no encontrado en el disco".to_string(),
        ));
    }

    let named_file = NamedFile::open(file_path)
        .map_err(|e| ApiError::InternalError(format!("Error opening file: {}", e)))?;

    use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
    let cd = ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Filename(record.original_name)],
    };

    Ok(Either::Right(named_file.set_content_disposition(cd)))
}

#[derive(Deserialize)]
pub struct RegistrarUrlInput {
    pub dni_asociado: String,
    pub original_name: String,
    pub external_url: String,
    pub documento_id: Option<i32>,
}

pub async fn registrar_url(
    data: web::Data<AppState>,
    input: web::Json<RegistrarUrlInput>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    if !input.external_url.starts_with("https://") {
        return Err(ApiError::BadRequest(
            "La URL externa debe usar HTTPS".to_string(),
        ));
    }

    if input.dni_asociado.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "El campo dni_asociado es obligatorio".to_string(),
        ));
    }

    if !input.original_name.to_lowercase().ends_with(".pdf") {
        return Err(ApiError::BadRequest(
            "El nombre del archivo debe terminar en .pdf".to_string(),
        ));
    }

    let claims = req.extensions().get::<Claims>().cloned();
    let usuario_subida = claims
        .map(|c| c.nombre)
        .unwrap_or_else(|| "Desconocido".to_string());

    let extension = "pdf";

    let result = sqlx::query!(
        r#"
        INSERT INTO fileserver (documento_id, dni_asociado, original_name, external_url, extension, usuario_subida)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        input.documento_id,
        input.dni_asociado,
        input.original_name,
        input.external_url,
        extension,
        usuario_subida
    )
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    let id = result.last_insert_id();

    let row = sqlx::query!(
        r#"SELECT CAST(file_hash AS CHAR) AS file_hash FROM fileserver WHERE id = ?"#,
        id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| ApiError::InternalError(format!("Database error: {}", e)))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": id,
        "original_name": input.original_name,
        "file_hash": row.file_hash,
        "external_url": input.external_url,
        "extension": extension,
    })))
}
