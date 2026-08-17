# Diseño: Refactor módulo Grado Académico — Arquitectura Limpia

**Fecha:** 2026-05-31  
**Módulo piloto:** Grado Académico  
**Objetivo:** Establecer el patrón de arquitectura limpia que se replicará en los demás módulos.

---

## Arquitectura objetivo

```
routes/personal.rs
    └── handlers/grado.rs        ← NUEVO (extraído de handlers/personal.rs)
            └── services/gradoaca_service.rs   ← se agrega get_por_dni
                    └── repositories/gradoaca_repo.rs  ← sin cambios
```

La regla que establece este módulo como plantilla:

> **Handler → Service → Repository**. Ningún handler importa un `_repo` directamente.

---

## Cambios por archivo

### 1. `services/gradoaca_service.rs` — agregar función faltante

Agregar:

```rust
pub async fn get_por_dni(db: &MySqlPool, dni: &str) -> Result<Vec<GradoAcademico>, ApiError> {
    gradoaca_repo::gradoacademico_por_dni(db, dni)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al obtener grados: {}", e)))
}
```

Esta función cierra el único bypass que existía (handler llamando al repo directamente).

---

### 2. `handlers/grado.rs` — archivo nuevo

Contenido: las 3 funciones extraídas de `handlers/personal.rs`:

- `grado_por_dni` — llama a `gradoaca_service::get_por_dni` (corregido)
- `upsert_gradoacademico` — llama a `gradoaca_service::upsert_gradoacademico` (sin cambios)
- `eliminar_gradoa` — llama a `gradoaca_service::eliminar_gradoa` (sin cambios)

El DTO `EliminarGradoBody` se mueve aquí también.

El handler reutiliza `super::registrar_historial` (el helper que ya existe en `handlers/mod.rs`) para auditoría — sin cambios en ese mecanismo.

---

### 3. `handlers/personal.rs` — eliminar código de grado

Remover:
- Las 3 funciones handler de grado
- El struct `EliminarGradoBody`
- El import de `repositories::gradoaca_repo`

---

### 4. `handlers/mod.rs` — declarar el nuevo módulo

Agregar: `pub mod grado;`

---

### 5. `routes/personal.rs` — actualizar imports

Cambiar los 3 imports de grado de `handlers::personal::*` a `handlers::grado::*`.

Las URLs no cambian:
- `POST /personal/grado_por_dni`
- `POST /personal/agregar_gradoa`
- `POST /personal/eliminar_gradoa`

---

## Lo que NO cambia

| Archivo | Motivo |
|---|---|
| `repositories/gradoaca_repo.rs` | Ya está bien estructurado |
| `models/personal.rs` | `GradoAcademico` queda donde está |
| URLs de los endpoints | Sin breaking changes |
| Mecanismo de auditoría | Sigue usando `registrar_historial` de `handlers/mod.rs` |

---

## Patrón resultante (plantilla para otros módulos)

Este refactor produce el patrón a replicar:

```
handlers/<dominio>.rs
    - Imports: solo services, models, middleware
    - Sin imports de repositories
    - DTOs de request definidos en el mismo archivo
    - Cada función: validar → llamar service → responder HTTP
```
