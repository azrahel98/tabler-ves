# Grado Académico — Plan de implementación (Arquitectura Limpia)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extraer el módulo de grado académico de `handlers/personal.rs` a su propio archivo, corrigiendo el bypass de la capa service.

**Architecture:** Tres capas estrictas: `handlers/grado.rs` → `services/gradoaca_service.rs` → `repositories/gradoaca_repo.rs`. Ningún handler importa un `_repo` directamente. Este módulo sirve como plantilla para los demás.

**Tech Stack:** Rust, actix-web, sqlx (MySQL), validator

---

## Mapa de archivos

| Archivo | Acción | Responsabilidad |
|---|---|---|
| `src/services/gradoaca_service.rs` | Modificar | Agregar `get_por_dni` |
| `src/handlers/grado.rs` | Crear | Los 3 handlers de grado + DTO |
| `src/handlers/mod.rs` | Modificar | Declarar `pub mod grado` |
| `src/handlers/personal.rs` | Modificar | Eliminar código de grado |
| `src/routes/personal.rs` | Modificar | Importar desde `handlers::grado` |

---

## Task 1: Agregar `get_por_dni` al service

**Files:**
- Modify: `src/services/gradoaca_service.rs`

- [ ] **Step 1: Agregar la función al final del archivo**

El archivo actual termina en la línea 83. Agregar al final:

```rust
pub async fn get_por_dni(db: &MySqlPool, dni: &str) -> Result<Vec<GradoAcademico>, ApiError> {
    gradoaca_repo::gradoacademico_por_dni(db, dni)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener grados: {}", e)))
}
```

- [ ] **Step 2: Verificar que compila**

```bash
cargo check
```

Esperado: sin errores.

- [ ] **Step 3: Commit**

```bash
git add src/services/gradoaca_service.rs
git commit -m "feat(gradoaca): agregar get_por_dni al service"
```

---

## Task 2: Crear `handlers/grado.rs`

**Files:**
- Create: `src/handlers/grado.rs`

- [ ] **Step 1: Crear el archivo con los 3 handlers**

Contenido completo de `src/handlers/grado.rs`:

```rust
use crate::{
    AppState,
    middleware::error::{ApiError, validar},
    models::personal::GradoAcademico,
    services::gradoaca_service,
};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use validator::Validate;
use super::registrar_historial;

#[derive(Deserialize, Validate)]
pub struct GradoDniBody {
    #[validate(custom(function = "crate::models::personal::es_dni_valido"))]
    pub dni: String,
}

pub async fn grado_por_dni(
    data: web::Data<AppState>,
    body: web::Json<GradoDniBody>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let gradoac = gradoaca_service::get_por_dni(&data.db, &body.dni).await?;
    Ok(HttpResponse::Ok().json(gradoac))
}

pub async fn upsert_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let (diff_value, accion, rows_affected) =
        gradoaca_service::upsert_gradoacademico(&data.db, &doc.0).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &doc.dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json(format!(
        "Operación exitosa. Filas afectadas: {}",
        rows_affected
    )))
}

#[derive(Deserialize, Validate)]
pub struct EliminarGradoBody {
    #[validate(range(min = 1, message = "ID de grado inválido"))]
    pub id: i32,
}

pub async fn eliminar_gradoa(
    data: web::Data<AppState>,
    body: web::Json<EliminarGradoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let (diff_value, accion, dni) = gradoaca_service::eliminar_gradoa(&data.db, body.id).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Grado académico eliminado"))
}
```

- [ ] **Step 2: Declarar el módulo en `handlers/mod.rs`**

Agregar `pub mod grado;` al archivo `src/handlers/mod.rs`. El archivo debe quedar así:

```rust
pub mod dash;
pub mod login;
pub mod personal;
pub mod usuarios;
pub mod fileserver;
pub mod grado;
use crate::middleware::error::ApiError;
use actix_web::{HttpMessage, HttpRequest};
pub async fn registrar_historial(
    req: &HttpRequest,
    db: &sqlx::MySqlPool,
    operacion: &str,
    dni: &str,
    detalle: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    use crate::models::audit::AuditContext;
    use crate::middleware::jwt::Claims;
    let audit_ctx = AuditContext {
        user_id: req
            .extensions()
            .get::<Claims>()
            .map(|claims| claims.id)
            .unwrap_or(0),
        ip: req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string(),
        user_agent: req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string(),
    };
    crate::services::audit_service::registrar_historial(db, &audit_ctx, operacion, dni, detalle).await
}
```

- [ ] **Step 3: Verificar que compila**

```bash
cargo check
```

Esperado: sin errores.

- [ ] **Step 4: Commit**

```bash
git add src/handlers/grado.rs src/handlers/mod.rs
git commit -m "feat(gradoaca): crear handlers/grado.rs con patron limpio"
```

---

## Task 3: Limpiar `handlers/personal.rs`

**Files:**
- Modify: `src/handlers/personal.rs`

- [ ] **Step 1: Eliminar el import de `gradoaca_repo`**

En la sección de imports al inicio del archivo (líneas 1-27), eliminar esta línea:

```rust
    repositories::gradoaca_repo,
```

- [ ] **Step 2: Eliminar las 3 funciones de grado y el DTO**

Eliminar del archivo los siguientes bloques (aproximadamente líneas 131-171):

```rust
pub async fn grado_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let gradoac = gradoaca_repo::gradoacademico_por_dni(&data.db, &dni.dni).await?;
    Ok(HttpResponse::Ok().json(gradoac))
}
pub async fn upsert_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let (diff_value, accion, rows_affected) =
        gradoaca_service::upsert_gradoacademico(&data.db, &doc.0).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &doc.dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json(format!(
        "Operación exitosa. Filas afectadas: {}",
        rows_affected
    )))
}
#[derive(Deserialize, Validate)]
pub struct EliminarGradoBody {
    #[validate(range(min = 1, message = "ID de grado inválido"))]
    pub id: i32,
}
pub async fn eliminar_gradoa(
    data: web::Data<AppState>,
    body: web::Json<EliminarGradoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let (diff_value, accion, dni) = gradoaca_service::eliminar_gradoa(&data.db, body.id).await?;
    if let Some(diff) = diff_value {
        let _ = registrar_historial(&req, &data.db, accion, &dni, Some(diff)).await;
    }
    Ok(HttpResponse::Ok().json("Grado académico eliminado"))
}
```

También eliminar `GradoAcademico` y `gradoaca_service` de los imports si ya no se usan en ninguna otra función del archivo.

- [ ] **Step 3: Verificar que compila**

```bash
cargo check
```

Esperado: sin errores. Si hay errores de imports no usados, eliminarlos también.

- [ ] **Step 4: Commit**

```bash
git add src/handlers/personal.rs
git commit -m "refactor(personal): extraer grado academico a su propio handler"
```

---

## Task 4: Actualizar `routes/personal.rs`

**Files:**
- Modify: `src/routes/personal.rs`

- [ ] **Step 1: Agregar import de `handlers::grado`**

Cambiar la primera línea del archivo de:

```rust
use crate::{handlers::personal::*, middleware::check::JWT};
```

a:

```rust
use crate::{handlers::personal::*, handlers::grado::*, middleware::check::JWT};
```

- [ ] **Step 2: Verificar que compila y linkea**

```bash
cargo build
```

Esperado: compilación exitosa sin warnings de símbolos no encontrados.

- [ ] **Step 3: Commit**

```bash
git add src/routes/personal.rs
git commit -m "refactor(routes): importar handlers de grado desde modulo dedicado"
```

---

## Task 5: Verificación final

- [ ] **Step 1: Build release para confirmar que no hay warnings ocultos**

```bash
cargo build --release 2>&1 | grep -E "warning|error"
```

Esperado: sin errores. Warnings de código no usado son aceptables si son preexistentes.

- [ ] **Step 2: Verificar que los 3 endpoints responden**

Levantar el servidor:

```bash
cargo run
```

Probar con curl (reemplazar `<TOKEN>` con un JWT válido):

```bash
# grado_por_dni
curl -s -X POST http://localhost:4010/personal/grado_por_dni \
  -H "Authorization: Bearer <TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"dni":"12345678"}' | head -c 200

# eliminar_gradoa (con id inexistente — debe devolver 404)
curl -s -X POST http://localhost:4010/personal/eliminar_gradoa \
  -H "Authorization: Bearer <TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"id":999999}'
```

Esperado: respuestas JSON (no HTML ni 500).
