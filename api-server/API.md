# API Endpoints

Base URL: `http://127.0.0.1:8080`

Todas las rutas protegidas requieren el header `token` con un JWT válido.

## Errores

Todas las rutas devuelven errores con la misma estructura:

```json
{ "code": 400 | 401 | 404 | 500, "error": "Mensaje descriptivo" }
```

---

## Login `/login`

### `POST /login/`

Autenticación de usuario.

**Body:**

| Campo      | Tipo   | Requerido |
|------------|--------|-----------|
| `username` | string | sí        |
| `password` | string | sí        |

**Respuesta:**

```json
{ "token": "eyJhbGciOi..." }
```

---

### `POST /login/changepass` 🔒

Cambiar contraseña.

**Body:**

| Campo     | Tipo   | Requerido |
|-----------|--------|-----------|
| `id`      | number | sí        |
| `oldpass` | string | sí        |
| `newpass` | string | sí        |

**Respuesta:**

```json
"Contraseña cambiada con éxito"
```

---

## Dashboard `/dash` 🔒

Todas las rutas de `/dash` requieren JWT.

### `POST /dash/info`

Resumen general del personal.

**Body:** vacío

**Respuesta:**

```json
{
  "total": 100,
  "activos": 80,
  "por_regimen": [{ "cantidad": 50, "nombre": "D.L. 276" }],
  "por_sexo": [{ "cantidad": 40, "nombre": "M" }],
  "por_sindicato": [{ "cantidad": 20, "nombre": "SITRAMUN" }]
}
```

---

### `POST /dash/cumpleaños`

Cumpleaños próximos.

**Body:** vacío

**Respuesta:**

```json
[
  { "dni": "12345678", "nombre": "Apellido Nombre", "nacimiento": "1990-01-15", "edad": 36 }
]
```

---

### `POST /dash/area_report`

Personal activo por área.

**Body:** vacío

**Respuesta:**

```json
[{ "cantidad": 10, "nombre": "Gerencia Municipal" }]
```

---

### `POST /dash/renuncias`

Renuncias del año en curso por área.

**Body:** vacío

**Respuesta:**

```json
[{ "cantidad": 3, "nombre": "Gerencia Municipal" }]
```

---

### `POST /dash/banco_report`

Lista de bancos.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "BCP" }]
```

---

### `POST /dash/personal_activo`

Reporte de personal activo completo.

**Body:** vacío

**Respuesta:**

```json
[
  {
    "dni": "12345678",
    "nombre": "Apellido Nombre",
    "ingreso": "2020-01-15",
    "renuncia": null,
    "area": "Gerencia",
    "cargo": "Analista",
    "sindicato": null,
    "regimen": "D.L. 276"
  }
]
```

---

### `POST /dash/reporte_historia`

Historial de operaciones filtrado por DNI.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
[
  { "operacion": "editar", "detalle": "...", "fecha": "2024-01-15 10:30:00", "nombre": "Admin" }
]
```

---

### `POST /dash/organigrama`

Organigrama jerárquico.

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1,
    "area": "Gerencia Municipal",
    "jefe": "Apellido Nombre",
    "dni": "12345678",
    "subgerencias": []
  }
]
```

---

### `POST /dash/reporte_legajos`

Legajos en estado préstamo.

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1, "nombre": "Apellido Nombre", "dni": "12345678",
    "estado": "prestamo", "persona": "Juan", "userid": 1,
    "usuario": "Admin", "fecha": "2024-01-15T10:30:00"
  }
]
```

---

### `POST /dash/reporte_renuncias`

Renuncias recientes (últimos 120 días).

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1, "dni": "12345678", "nombre": "Apellido Nombre",
    "fecha": "2024-01-15", "cargo": "Analista",
    "area": "Gerencia", "codigo": "P001"
  }
]
```

---

### `POST /dash/reporte_documentos`

Lista de tipos de documento.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Resolución", "sigla": "RA" }]
```

---

## Personal `/personal` 🔒

Todas las rutas de `/personal` requieren JWT.

### `POST /personal/buscar`

Buscar trabajadores por nombre.

**Body:**

| Campo    | Tipo   | Requerido |
|----------|--------|-----------|
| `nombre` | string | sí        |

**Respuesta:**

```json
[
  { "nombre": "Nombre Apellido", "dni": "12345678", "estado": "activo", "sexo": "M" }
]
```

---

### `POST /personal/por_dni`

Perfil de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
{
  "dni": "12345678", "nombre": "Nombre Apellido",
  "telf": "999999999", "direccion": "Av. Ejemplo 123",
  "email": "correo@mail.com", "ruc": "10123456789",
  "nacimiento": "1990-01-15", "sexo": "M"
}
```

---

### `POST /personal/editar_por_dni`

Editar datos personales.

**Body:**

| Campo       | Tipo    | Requerido |
|-------------|---------|-----------|
| `dni`       | string  | sí        |
| `nombre`    | string? | no        |
| `telf`      | string? | no        |
| `direccion` | string? | no        |
| `email`     | string? | no        |
| `ruc`       | string? | no        |
| `nacimiento`| string  | sí        |
| `sexo`      | string? | no        |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/vinculos_por_dni`

Vínculos laborales de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
[
  {
    "id": 1, "dni": "12345678", "doc_ingreso": "RA",
    "numero_doc_ingreso": "001", "descrip_ingreso": "...",
    "fecha_ingreso": "2020-01-15", "area": "Gerencia",
    "cargo": "Analista", "regimen": "D.L. 276",
    "sueldo": 2500.0, "codigo": "P001",
    "cargo_estructural": "...", "grupo_ocupacional": "...",
    "estado": "activo", "doc_salida": null,
    "descrip_salida": null, "fecha_salida": null,
    "numero_doc_salida": null, "sindicato": null,
    "tipo_evento": null, "estado_evento": null,
    "doc_evento_tipo": null, "numero_doc_evento": null,
    "fecha_evento": null
  }
]
```

---

### `POST /personal/renuncia_por_vinculo`

Registrar renuncia de un vínculo.

**Body:**

| Campo            | Tipo    | Requerido |
|------------------|---------|-----------|
| `id`             | number? | sí (vinculo id) |
| `tipoDocumento`  | string? | no        |
| `numeroDocumento`| number? | no        |
| `añoDocumento`   | number? | no        |
| `fecha`          | string  | sí        |
| `fechaValida`    | string? | no        |
| `descripcion`    | string  | sí        |

**Respuesta:**

```json
{
  "dni": "12345678", "nombre": "Analista",
  "estado": "inactivo", "fecha": "2024-06-15",
  "descripcion": "Renuncia voluntaria", "documento": "RA-001-2024"
}
```

---

### `POST /personal/banco_por_dni`

Datos bancarios de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
{
  "id": 1, "numero_cuenta": "123456789",
  "tipo_cuenta": "AHORRO", "cci": "00212345678901234567",
  "banco": "BCP", "estado": 1, "dni": "12345678"
}
```

---

### `POST /personal/agregar_infobancaria`

Agregar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido |
|-----------------|---------|-----------|
| `numero_cuenta` | string  | sí        |
| `tipo_cuenta`   | string  | sí        |
| `cci`           | string? | no        |
| `banco`         | number  | sí (id)   |
| `estado`        | number  | sí        |
| `dni`           | string  | sí        |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/editar_infobancaria`

Editar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido |
|-----------------|---------|-----------|
| `id`            | number  | sí        |
| `numero_cuenta` | string  | sí        |
| `tipo_cuenta`   | string? | no        |
| `cci`           | string? | no        |
| `banco`         | string  | sí        |
| `estado`        | number  | sí        |
| `dni`           | string  | sí        |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/grado_por_dni`

Grado académico de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
{
  "id": 1, "profesion": "Ingeniero de Sistemas",
  "universidad": "UNMSM", "colegiatura": "CIP 12345",
  "nivel_academico": "Titulado", "abrv": "Ing.", "dni": "12345678"
}
```

---

### `POST /personal/agregar_gradoa`

Agregar o actualizar grado académico (upsert). Si `id` es `0` se inserta, si `id > 0` se actualiza.

**Body:**

| Campo            | Tipo    | Requerido |
|------------------|---------|-----------|
| `id`             | number  | sí        |
| `profesion`      | string  | sí        |
| `universidad`    | string  | sí        |
| `colegiatura`    | string? | no        |
| `nivel_academico`| string  | sí        |
| `abrv`           | string  | sí        |
| `dni`            | string  | sí        |

**Respuesta:**

```json
"Operación exitosa. Filas afectadas: 1"
```

---

### `POST /personal/agregar_sindicato`

Afiliar vínculo a sindicato.

**Body:**

| Campo            | Tipo    | Requerido |
|------------------|---------|-----------|
| `tipoDocumento`  | string? | no        |
| `numeroDocumento`| number? | no        |
| `añoDocumento`   | number? | no        |
| `fecha`          | string  | sí        |
| `fechaValida`    | string? | no        |
| `descripcion`    | string  | sí        |
| `sindicato`      | number  | sí (id)   |
| `id_vinculo`     | number  | sí        |

**Respuesta:**

```json
"Se registraron correctamente los datos"
```

---

### `POST /personal/personas_legajo`

Lista de personas con registros en legajo.

**Body:** vacío

**Respuesta:**

```json
[{ "persona": "Juan Pérez" }]
```

---

### `POST /personal/legajo_por_dni`

Registros de legajo de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
[
  {
    "id": 1, "persona": "Juan Pérez", "dni": "12345678",
    "fecha": "2024-01-15", "estado": "prestamo",
    "descrip": "Legajo completo", "nuevo": 0, "user": 1
  }
]
```

---

### `POST /personal/nuevo_evento_legajo`

Agregar evento de legajo.

**Body:**

| Campo    | Tipo    | Requerido |
|----------|---------|-----------|
| `id`     | number  | sí        |
| `persona`| string  | sí        |
| `dni`    | string  | sí        |
| `fecha`  | string? | no        |
| `estado` | string? | no        |
| `descrip`| string? | no        |
| `nuevo`  | number  | sí (1 = nuevo estado) |
| `user`   | number? | no        |

**Respuesta:**

```json
"Se registraron correctamente los datos"
```

---

### `POST /personal/asistencia`

Reporte de asistencia mensual.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |
| `mes` | number | sí        |
| `año` | number | sí        |

**Respuesta:**

```json
[{ "dni": "12345678", "hora": "08:00", "fecha": "2024-01-15" }]
```

---

### `POST /personal/agregar_contacto`

Agregar o actualizar contacto de emergencia.

**Body:**

| Campo        | Tipo    | Requerido |
|--------------|---------|-----------|
| `persona_dni`| string  | sí        |
| `nombre`     | string  | sí        |
| `relacion`   | string  | sí        |
| `telefono`   | string? | no        |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/contacto_dni`

Contacto de emergencia de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
|-------|--------|-----------|
| `dni` | string | sí        |

**Respuesta:**

```json
{ "persona_dni": "12345678", "nombre": "María", "relacion": "Madre", "telefono": "999999999" }
```

---

### `POST /personal/buscar_vacantes`

Vacantes recientes (últimos 3 meses).

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1, "dni": "12345678", "nombre": "Apellido Nombre",
    "fecha": "2024-06-15", "fechavalida": "2024-06-20",
    "area": "Gerencia", "cargo": "Analista",
    "codigo": "P001", "sueldo": 2500.0
  }
]
```

---

### `POST /personal/buscar_por_plaza`

Detalle de una plaza por código.

**Body:**

| Campo   | Tipo   | Requerido |
|---------|--------|-----------|
| `codigo`| string | sí        |

**Respuesta:**

```json
{
  "codigo": "P001", "cargo_estructural": "CE001",
  "cargo_descripcion": "Analista", "grupo_ocupacional": "GO001",
  "grupo_descripcion": "Profesional", "condicion": "Nombrado",
  "regimen_id": 1, "regimen": "D.L. 276"
}
```
