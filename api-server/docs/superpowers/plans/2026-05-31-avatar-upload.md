# Avatar Upload — Plan de implementación

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Agregar endpoint para subir foto de perfil (base64 → PNG en disco) y servirla, guardando la ruta relativa en `persona.avatar`.

**Architecture:** Tres capas existentes sin archivos nuevos: `handlers/personal.rs` → `services/personal_service.rs` → `repositories/personal_repo.rs`. El archivo PNG se guarda en `{UPLOAD_DIR}/AVATARS/{dni}.png` y se reemplaza en cada subida.

**Tech Stack:** Rust, actix-web 4, actix-files 0.6, sqlx 0.7 (MySQL), base64 0.22

---

## Mapa de archivos

| Archivo | Acción |
|---|---|
| `Cargo.toml` | Agregar `base64 = "0.22"` |
| `src/repositories/personal_repo.rs` | Agregar `actualizar_avatar` |
| `src/services/personal_service.rs` | Agregar `guardar_avatar` |
| `src/handlers/personal.rs` | Agregar `AvatarPayload`, `subir_avatar`, `ver_avatar` |
| `src/routes/personal.rs` | Registrar 2 rutas |

---

## Task 1: Agregar dependencia base64

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Agregar la dependencia**

En `Cargo.toml`, agregar después de la línea `calamine = ...`:

```toml
base64 = "0.22"
```

- [ ] **Step 2: Verificar que descarga y compila**

```bash
cargo check
```

Esperado: sin errores (descarga el crate).

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: agregar dependencia base64"
```

---

## Task 2: Agregar `actualizar_avatar` al repo

**Files:**
- Modify: `src/repositories/personal_repo.rs`

- [ ] **Step 1: Agregar la función al final del archivo**

El archivo termina con `consultar_dni_local`. Agregar después:

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

- [ ] **Step 2: Verificar compilación**

```bash
cargo check
```

Esperado: sin errores.

- [ ] **Step 3: Commit**

```bash
git add src/repositories/personal_repo.rs
git commit -m "feat(avatar): agregar actualizar_avatar al repo"
```

---

## Task 3: Agregar `guardar_avatar` al service

**Files:**
- Modify: `src/services/personal_service.rs`

- [ ] **Step 1: Agregar el import de base64 al inicio del archivo**

El archivo comienza con varios `use`. Agregar esta línea entre los imports existentes:

```rust
use base64::Engine;
```

El bloque de imports completo debe quedar así:

```rust
use crate::middleware::error::ApiError;
use crate::models::personal::{Perfil, PerfilInput, Persona, Vinculos};
use base64::Engine;
use chrono::NaiveDate;
use reqwest::Client;
use serde_json::Value;
use crate::repositories::personal_repo;
use sqlx::MySqlPool;
```

- [ ] **Step 2: Agregar la función al final del archivo**

Después de `consultar_dni_reniec`, agregar:

```rust
pub async fn guardar_avatar(
    db: &MySqlPool,
    dni: &str,
    imagen_base64: &str,
) -> Result<String, ApiError> {
    let base64_limpio = if let Some(pos) = imagen_base64.find("base64,") {
        &imagen_base64[pos + 7..]
    } else {
        imagen_base64
    };
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_limpio)
        .map_err(|_| ApiError::BadRequest("La imagen base64 no es válida".into()))?;
    if bytes.len() > 2_097_152 {
        return Err(ApiError::BadRequest("La imagen no debe superar 2 MB".into()));
    }
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let avatars_dir = std::path::PathBuf::from(&upload_dir).join("AVATARS");
    std::fs::create_dir_all(&avatars_dir)
        .map_err(|e| ApiError::InternalError(format!("Error creando directorio: {}", e)))?;
    let file_path = avatars_dir.join(format!("{}.png", dni));
    std::fs::write(&file_path, &bytes)
        .map_err(|e| ApiError::InternalError(format!("Error guardando imagen: {}", e)))?;
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

- [ ] **Step 3: Verificar compilación**

```bash
cargo check
```

Esperado: sin errores.

- [ ] **Step 4: Commit**

```bash
git add src/services/personal_service.rs
git commit -m "feat(avatar): agregar guardar_avatar al service"
```

---

## Task 4: Agregar handlers al handler de personal

**Files:**
- Modify: `src/handlers/personal.rs`

- [ ] **Step 1: Agregar import de `actix_files`**

En `handlers/personal.rs`, los imports de `actix_web` están al inicio. Agregar esta línea después de los imports existentes (antes del primer `#[derive...]`):

```rust
use actix_files::NamedFile;
```

- [ ] **Step 2: Agregar el DTO y los dos handlers al final del archivo**

Al final de `handlers/personal.rs` (después de `calidad_datos`), agregar:

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
    let avatar_url =
        personal_service::guardar_avatar(&data.db, &body.dni, &body.imagen_base64).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "avatar": avatar_url })))
}

pub async fn ver_avatar(
    path: web::Path<String>,
) -> Result<NamedFile, ApiError> {
    let dni = path.into_inner();
    if dni.len() != 8 || !dni.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::BadRequest("DNI inválido".into()));
    }
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let file_path = std::path::PathBuf::from(upload_dir)
        .join("AVATARS")
        .join(format!("{}.png", dni));
    if !file_path.exists() {
        return Err(ApiError::NotFound("Avatar no encontrado".into()));
    }
    NamedFile::open(file_path)
        .map_err(|e| ApiError::InternalError(format!("Error abriendo archivo: {}", e)))
}
```

- [ ] **Step 3: Verificar compilación**

```bash
cargo check
```

Esperado: sin errores.

- [ ] **Step 4: Commit**

```bash
git add src/handlers/personal.rs
git commit -m "feat(avatar): agregar handlers subir_avatar y ver_avatar"
```

---

## Task 5: Registrar rutas

**Files:**
- Modify: `src/routes/personal.rs`

- [ ] **Step 1: Agregar las 2 rutas al scope `/personal`**

En `src/routes/personal.rs`, dentro del bloque `.service(web::scope("/personal")...)`, agregar antes del cierre `)`:

```rust
.route("/avatar", web::post().to(subir_avatar))
.route("/avatar/{dni}", web::get().to(ver_avatar))
```

El final del scope debe quedar así:

```rust
            .route("/cambio_area", web::post().to(registrar_cambio_area))
            .route("/calidad_datos", web::post().to(calidad_datos))
            .route("/avatar", web::post().to(subir_avatar))
            .route("/avatar/{dni}", web::get().to(ver_avatar)),
    );
```

- [ ] **Step 2: Build completo**

```bash
cargo build
```

Esperado: compilación exitosa, 0 errores.

- [ ] **Step 3: Commit**

```bash
git add src/routes/personal.rs
git commit -m "feat(avatar): registrar rutas POST y GET de avatar"
```

---

## Task 6: Verificación manual

- [ ] **Step 1: Levantar el servidor**

```bash
cargo run
```

- [ ] **Step 2: Probar subida de avatar (reemplazar TOKEN y DNI)**

```bash
curl -s -X POST http://localhost:4010/personal/avatar \
  -H "Authorization: Bearer <TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{
    "dni": "12345678",
    "imagen_base64": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
  }'
```

Esperado:
```json
{ "avatar": "/avatars/12345678.png" }
```

- [ ] **Step 3: Confirmar que el archivo existe en disco**

Verificar que existe: `{UPLOAD_DIR}/AVATARS/12345678.png`

- [ ] **Step 4: Probar servicio de imagen**

```bash
curl -s -o avatar_test.png \
  -H "Authorization: Bearer <TOKEN>" \
  http://localhost:4010/personal/avatar/12345678
file avatar_test.png
```

Esperado: `avatar_test.png: PNG image data`

- [ ] **Step 5: Probar DNI sin avatar**

```bash
curl -s -X GET http://localhost:4010/personal/avatar/99999999 \
  -H "Authorization: Bearer <TOKEN>"
```

Esperado: respuesta 404.
