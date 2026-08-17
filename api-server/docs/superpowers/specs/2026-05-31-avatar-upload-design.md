# Diseño: Subida y servicio de foto de perfil (avatar)

**Fecha:** 2026-05-31

---

## Goal

Permitir subir una foto de perfil por trabajador (vía base64) y servirla como imagen PNG.

## Architecture

Tres capas: `handlers/personal.rs` → `services/personal_service.rs` → `repositories/personal_repo.rs`. Sin archivos nuevos.

**Tech Stack:** Rust, actix-web, sqlx (MySQL), base64 crate

---

## Endpoints

### POST `/personal/avatar`

**Request:**
```json
{ "dni": "12345678", "imagen_base64": "iVBORw0KGgo..." }
```
El campo `imagen_base64` puede incluir o no el prefijo `data:image/...;base64,` — se normaliza antes de decodificar.

**Flujo:**
1. Validar DNI (8 dígitos numéricos, usando `es_dni_valido`)
2. Strip del prefijo `data:image/...;base64,` si está presente
3. Decodificar base64 → bytes
4. Validar tamaño ≤ 2 MB (2_097_152 bytes)
5. Crear directorio `UPLOAD_DIR/AVATARS/` si no existe
6. Escribir bytes en `{UPLOAD_DIR}/AVATARS/{dni}.png` (reemplaza si ya existe)
7. Llamar a `personal_repo::actualizar_avatar(db, dni, "/avatars/{dni}.png")`
8. Devolver `{ "avatar": "/avatars/12345678.png" }`

**Errores:**
- DNI inválido → 400
- Base64 inválido → 400
- Imagen > 2 MB → 400
- Error de disco o BD → 500

### GET `/personal/avatar/{dni}`

**Flujo:**
1. Validar que `dni` tiene 8 dígitos (extraído del path)
2. Construir ruta: `{UPLOAD_DIR}/AVATARS/{dni}.png`
3. Si el archivo no existe → 404
4. Servir como `image/png` con `NamedFile`

---

## Cambios por archivo

### `repositories/personal_repo.rs`

Agregar:
```rust
pub async fn actualizar_avatar(db: &MySqlPool, dni: &str, url: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE persona SET avatar = ? WHERE dni = ?",
        url,
        dni
    )
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}
```

### `services/personal_service.rs`

Agregar:
```rust
pub async fn guardar_avatar(db: &MySqlPool, dni: &str, imagen_base64: &str) -> Result<String, ApiError> {
    // 1. Strip prefijo data URI si está presente
    let base64_limpio = if let Some(pos) = imagen_base64.find("base64,") {
        &imagen_base64[pos + 7..]
    } else {
        imagen_base64
    };

    // 2. Decodificar
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_limpio)
        .map_err(|_| ApiError::BadRequest("La imagen base64 no es válida".into()))?;

    // 3. Validar tamaño
    if bytes.len() > 2_097_152 {
        return Err(ApiError::BadRequest("La imagen no debe superar 2 MB".into()));
    }

    // 4. Guardar en disco
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let avatars_dir = std::path::PathBuf::from(&upload_dir).join("AVATARS");
    std::fs::create_dir_all(&avatars_dir)
        .map_err(|e| ApiError::InternalError(format!("Error creando directorio: {}", e)))?;

    let file_path = avatars_dir.join(format!("{}.png", dni));
    std::fs::write(&file_path, &bytes)
        .map_err(|e| ApiError::InternalError(format!("Error guardando imagen: {}", e)))?;

    // 5. Actualizar BD
    let avatar_url = format!("/avatars/{}.png", dni);
    personal_repo::actualizar_avatar(db, dni, &avatar_url)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Error al actualizar avatar en BD".into())
        })?;

    Ok(avatar_url)
}
```

### `handlers/personal.rs`

Agregar DTO y 2 funciones handler:

```rust
#[derive(Deserialize, Validate)]
pub struct AvatarPayload {
    #[validate(custom(function = "crate::models::personal::es_dni_valido"))]
    pub dni: String,
    #[validate(length(min = 1, message = "La imagen no puede estar vacía"))]
    pub imagen_base64: String,
}

pub async fn subir_avatar(
    data: web::Data<AppState>,
    body: web::Json<AvatarPayload>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let avatar_url = personal_service::guardar_avatar(&data.db, &body.dni, &body.imagen_base64).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "avatar": avatar_url })))
}

pub async fn ver_avatar(
    path: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let dni = path.into_inner();
    if dni.len() != 8 || !dni.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::BadRequest("DNI inválido".into()));
    }
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let file_path = std::path::PathBuf::from(upload_dir).join("AVATARS").join(format!("{}.png", dni));
    if !file_path.exists() {
        return Err(ApiError::NotFound("Avatar no encontrado".into()));
    }
    let named_file = actix_files::NamedFile::open(file_path)
        .map_err(|e| ApiError::InternalError(format!("Error abriendo archivo: {}", e)))?;
    Ok(named_file)
}
```

### `routes/personal.rs`

Agregar dentro del scope `/personal`:
```rust
.route("/avatar", web::post().to(subir_avatar))
.route("/avatar/{dni}", web::get().to(ver_avatar))
```

---

## Dependencia nueva

Agregar al `Cargo.toml`:
```toml
base64 = "0.22"
```

---

## Notas

- El archivo PNG se reemplaza en cada subida — no acumula versiones anteriores.
- Ambas rutas están dentro del scope `/personal` que tiene `.wrap(JWT)`, por lo que requieren token válido.
- La columna `persona.avatar` ya existe en la BD.
