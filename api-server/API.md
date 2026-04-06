# API Endpoints

Base URL: `http://127.0.0.1:4010`

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
| ---------- | ------ | --------- |
| `username` | string | sí        |
| `password` | string | sí        |

**Respuesta:**

```json
{ "token": "eyJhbGciOi..." }
```

---

### `POST /login/changepass` 🔒

Cambiar contraseña propia.

**Body:**

| Campo     | Tipo   | Requerido |
| --------- | ------ | --------- |
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

Cumpleaños próximos (±5 días pasados / +30 días futuros).

**Body:** vacío

**Respuesta:**

```json
[{ "dni": "12345678", "nombre": "Apellido Nombre", "nacimiento": "1990-01-15", "edad": 36 }]
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
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[{ "operacion": "editar", "detalle": "...", "fecha": "2024-01-15 10:30:00", "nombre": "Admin" }]
```

---

### `POST /dash/organigrama`

Organigrama jerárquico.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "area": "Gerencia Municipal", "jefe": "Apellido Nombre", "dni": "12345678", "subgerencias": [] }]
```

---

### `POST /dash/reporte_renuncias`

Renuncias recientes (últimos 120 días).

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", "fecha": "2024-01-15", "cargo": "Analista", "area": "Gerencia", "codigo": "P001" }]
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

### `POST /dash/activos_por_distrito`

Cantidad de personal activo agrupado por distrito de residencia.

**Body:** vacío

**Respuesta:**

```json
[{ "distrito": "PIURA", "cantidad": 45 }, { "distrito": "SIN ASIGNAR", "cantidad": 5 }]
```

---

### `POST /dash/exportar_excel`

Genera y descarga un archivo Excel con el padrón de personal activo y pendiente.

**Body:** vacío

**Respuesta:** `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` (binario)

---

### `POST /dash/nuevos_trabajadores`

Trabajadores ingresados en los últimos 120 días.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", "ingreso": "2024-01-15", "documento": "RA-001-2024", "area": "Gerencia", "cargo": "Analista", "regimen": "D.L. 276", "sueldo": 2500.0, "plaza": "P001" }]
```

---

### `POST /dash/reporte_eventos`

Todos los eventos de vínculo registrados (rotaciones, abandonos).

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1,
    "tipo_evento": "rotacion",
    "estado": "activo",
    "nombre": "Apellido Nombre",
    "dni": "12345678",
    "area_original": "Gerencia",
    "area_nueva": "Subgerencia",
    "cargo": "Analista",
    "fecha_inicio": "2024-01-15",
    "descripcion_inicio": "...",
    "fecha_salida": null,
    "descripcion_salida": null
  }
]
```

---

## Personal `/personal` 🔒

Todas las rutas de `/personal` requieren JWT.

### `POST /personal/buscar`

Buscar trabajadores por nombre.

**Body:**

| Campo    | Tipo   | Requerido |
| -------- | ------ | --------- |
| `nombre` | string | sí        |

**Respuesta:**

```json
[{ "nombre": "Nombre Apellido", "dni": "12345678", "estado": "activo", "sexo": "M" }]
```

---

### `POST /personal/por_dni`

Perfil de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
{
  "dni": "12345678",
  "nombre": "Nombre Apellido",
  "telf": "999999999",
  "direccion": "Av. Ejemplo 123",
  "email": "correo@mail.com",
  "ruc": "10123456789",
  "nacimiento": "1990-01-15",
  "sexo": "M",
  "region": "PIURA",
  "distrito": "PIURA"
}
```

---

### `POST /personal/editar_por_dni`

Editar datos personales.

**Body:**

| Campo        | Tipo    | Requerido |
| ------------ | ------- | --------- |
| `dni`        | string  | sí        |
| `nombre`     | string? | no        |
| `telf`       | string? | no        |
| `direccion`  | string? | no        |
| `email`      | string? | no        |
| `ruc`        | string? | no        |
| `nacimiento` | string  | sí        |
| `sexo`       | string? | no        |
| `region`     | string? | no        |
| `distrito`   | string? | no        |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/vinculos_por_dni`

Vínculos laborales de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[
  {
    "id": 1,
    "dni": "12345678",
    "area": "Gerencia",
    "cargo": "Analista",
    "regimen": "D.L. 276",
    "sueldo": 2500.0,
    "codigo": "P001",
    "estado": "activo",
    "fecha_ingreso": "2020-01-15",
    "fecha_salida": null,
    "sindicato": null,
    "tipo_evento": null,
    "estado_evento": null,
    "id_evento": null
  }
]
```

---

### `POST /personal/renuncia_por_vinculo`

Registrar renuncia de un vínculo.

**Body:**

| Campo             | Tipo    | Requerido       |
| ----------------- | ------- | --------------- |
| `id`              | number  | sí (vinculo id) |
| `tipoDocumento`   | string? | no              |
| `numeroDocumento` | number? | no              |
| `añoDocumento`    | number? | no              |
| `fecha`           | string  | sí              |
| `descripcion`     | string  | sí              |

**Respuesta:**

```json
{ "dni": "12345678", "estado": "inactivo" }
```

---

### `POST /personal/eliminar_vinculo`

Eliminar un vínculo laboral y sus documentos asociados.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `id`  | number | sí        |

**Respuesta:**

```json
"Vínculo eliminado correctamente"
```

---

### `POST /personal/banco_por_dni`

Datos bancarios de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
{ "id": 1, "numero_cuenta": "123456789", "tipo_cuenta": "AHORRO", "cci": "00212345678901234567", "banco": "BCP", "estado": 1 }
```

---

### `POST /personal/agregar_infobancaria`

Agregar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido |
| --------------- | ------- | --------- |
| `numero_cuenta` | string  | sí        |
| `tipo_cuenta`   | string  | sí        |
| `cci`           | string? | no        |
| `banco`         | number  | sí (id)   |
| `estado`        | number  | sí        |
| `dni`           | string  | sí        |

**Respuesta:** `"Rows affected: 1"`

---

### `POST /personal/editar_infobancaria`

Editar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido |
| --------------- | ------- | --------- |
| `id`            | number  | sí        |
| `numero_cuenta` | string  | sí        |
| `tipo_cuenta`   | string? | no        |
| `cci`           | string? | no        |
| `banco`         | string  | sí        |
| `estado`        | number  | sí        |
| `dni`           | string  | sí        |

**Respuesta:** `"Rows affected: 1"`

---

### `POST /personal/grado_por_dni`

Grados académicos de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[{ "id": 1, "profesion": "ABOGADO", "universidad": "UNIV.", "colegiatura": null, "nivel_academico": "TITULADO", "abrv": "ABOG", "dni": "12345678" }]
```

---

### `POST /personal/agregar_gradoa`

Agregar o actualizar grado académico (upsert).

**Body:**

| Campo             | Tipo    | Requerido                    |
| ----------------- | ------- | ---------------------------- |
| `id`              | number  | sí (0 = insertar, >0 = editar) |
| `profesion`       | string  | sí                           |
| `universidad`     | string  | sí                           |
| `colegiatura`     | string? | no                           |
| `nivel_academico` | string  | sí                           |
| `abrv`            | string  | sí                           |
| `dni`             | string  | sí                           |

**Respuesta:** `"Operación exitosa. Filas afectadas: 1"`

---

### `POST /personal/agregar_contacto`

Agregar o actualizar contacto de emergencia (upsert por DNI).

**Body:**

| Campo         | Tipo    | Requerido |
| ------------- | ------- | --------- |
| `persona_dni` | string  | sí        |
| `nombre`      | string  | sí        |
| `relacion`    | string  | sí        |
| `telefono`    | string? | no        |

**Respuesta:** `"Rows affected: 1"`

---

### `POST /personal/contacto_dni`

Contacto de emergencia de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
{ "persona_dni": "12345678", "nombre": "María", "relacion": "Madre", "telefono": "999999999" }
```

---

### `POST /personal/eliminar_contacto`

Eliminar el contacto de emergencia de un trabajador.

**Body:**

| Campo         | Tipo   | Requerido |
| ------------- | ------ | --------- |
| `persona_dni` | string | sí        |

**Respuesta:** `"Contacto de emergencia eliminado"`

---

### `POST /personal/agregar_sindicato`

Afiliar vínculos a un sindicato.

**Body:**

| Campo             | Tipo    | Requerido |
| ----------------- | ------- | --------- |
| `fecha`           | string  | sí        |
| `descripcion`     | string  | sí        |
| `sindicato`       | number  | sí (id)   |
| `vinculos`        | array   | sí        |

**Estructura de `vinculos`:**

| Campo        | Tipo   |
| ------------ | ------ |
| `id_vinculo` | number |
| `dni`        | string |

**Respuesta:** `"Se registraron correctamente los datos"`

---

### `POST /personal/eliminar_sindicato`

Desafiliar un vínculo de su sindicato.

**Body:**

| Campo        | Tipo   | Requerido |
| ------------ | ------ | --------- |
| `vinculo_id` | number | sí        |
| `dni`        | string | sí        |

**Respuesta:** `"Afiliación sindical eliminada"`

---

### `POST /personal/legajo_por_dni`

Registros de legajo de un trabajador.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[{ "id": 1, "persona": "Juan Pérez", "dni": "12345678", "fecha": "2024-01-15", "estado": "prestamo", "descrip": "Legajo completo", "nuevo": 0, "user": 1 }]
```

---

### `POST /personal/nuevo_evento_legajo`

Agregar evento de legajo.

**Body:**

| Campo     | Tipo    | Requerido |
| --------- | ------- | --------- |
| `id`      | number  | sí        |
| `persona` | string  | sí        |
| `dni`     | string  | sí        |
| `fecha`   | string? | no        |
| `estado`  | string? | no        |
| `descrip` | string? | no        |
| `nuevo`   | number  | sí        |
| `user`    | number? | no        |

**Respuesta:** `"Se registraron correctamente los datos"`

---

### `POST /personal/asistencia`

Reporte de asistencia mensual.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |
| `mes` | number | sí        |
| `año` | number | sí        |

**Respuesta:**

```json
[{ "dni": "12345678", "hora": "08:00", "fecha": "2024-01-15" }]
```

---

### `POST /personal/buscar_vacantes`

Vacantes recientes (últimos 3 meses).

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "dni": null, "nombre": null, "fecha": "2024-06-15", "area": "Gerencia", "cargo": "Analista", "codigo": "P001", "sueldo": 2500.0 }]
```

---

### `POST /personal/buscar_por_plaza`

Detalle de una plaza por código.

**Body:**

| Campo    | Tipo   | Requerido |
| -------- | ------ | --------- |
| `codigo` | string | sí        |

**Respuesta:**

```json
{ "codigo": "P001", "cargo_estructural": "CE001", "cargo_descripcion": "Analista", "condicion": "Nombrado", "regimen_id": 1 }
```

---

### `POST /personal/registrar_trabajador`

Registrar un nuevo trabajador con persona, documento y vínculo.

**Body:**

| Campo       | Tipo   | Requerido         |
| ----------- | ------ | ----------------- |
| `personal`  | object | sí                |
| `airshp`    | string | sí (plaza código) |
| `documento` | object | sí                |
| `regimen`   | number | sí (id)           |
| `cargo`     | number | sí (id)           |
| `area`      | number | sí (id)           |
| `sueldo`    | number | sí                |

**Respuesta:** `"Trabajador registrado correctamente"`

---

### `POST /personal/consultar_dni`

Consultar datos por DNI. Busca localmente; si no existe, consulta la API de RENIEC.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
{ "dni": "12345678", "apaterno": "GARCIA", "amaterno": "LOPEZ", "nombre": "JUAN", "nacimiento": "1990-01-15", "sexo": "M" }
```

---

### `POST /personal/buscar_areas`

Lista de áreas activas.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Gerencia Municipal" }]
```

---

### `POST /personal/buscar_cargos`

Lista de cargos.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Analista" }]
```

---

### `POST /personal/upsert_evento_vinculo`

Agregar o actualizar un evento de vínculo (rotación, abandono). Si `id` es `null` se crea nuevo.

**Body:**

| Campo              | Tipo    | Requerido |
| ------------------ | ------- | --------- |
| `id`               | number? | no        |
| `vinculo_id`       | number  | sí        |
| `tipo_evento`      | string  | sí        |
| `nueva_area_id`    | number? | solo en rotación |
| `documento_inicio` | object? | no        |
| `documento_salida` | object? | no        |
| `estado`           | string? | no        |

**Respuesta:** `"Operación exitosa"`

---

### `POST /personal/delete_evento_vinculo`

Eliminar un evento de vínculo y sus documentos asociados.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `id`  | number | sí        |

**Respuesta:** `"Evento de vínculo eliminado"`

---

## Fileserver `/fileserver`

Las rutas requieren JWT **excepto `GET /fileserver/{hash}`**.

### `GET /fileserver/{hash}`

Acceder a un archivo por su hash. No requiere JWT (se puede usar en `<img>` o visor de PDF).

**Respuesta:** contenido binario del archivo.

---

### `POST /fileserver/upload` 🔒

Subir un archivo (máx. 10 MB).

**Body (multipart/form-data):**

| Campo          | Tipo   | Requerido |
| -------------- | ------ | --------- |
| `archivo/file` | file   | sí        |
| `dni_asociado` | string | sí        |
| `documento_id` | number | no        |

**Respuesta:**

```json
[{ "id": 1, "original_name": "mi_doc.pdf", "file_hash": "uuid...", "extension": "pdf" }]
```

---

### `POST /fileserver/listar_archivos_dni` 🔒

Lista de archivos de un DNI.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[{ "id": 1, "original_name": "mi_doc.pdf", "file_hash": "uuid...", "extension": "pdf", "usuario_subida": "Admin", "fecha_subida": "2024-01-15 10:30:00" }]
```

---

### `POST /fileserver/eliminar_archivo` 🔒

Eliminar un archivo por ID.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `id`  | number | sí        |

**Respuesta:** `"Archivo eliminado correctamente"`

---

### `POST /fileserver/asignar_documento` 🔒

Asignar un archivo a un documento de legajo.

**Body:**

| Campo          | Tipo   | Requerido |
| -------------- | ------ | --------- |
| `archivo_id`   | number | sí        |
| `documento_id` | number | sí        |

**Respuesta:** `"Documento asignado correctamente"`

---

### `POST /fileserver/documentos_por_dni` 🔒

Lista de documentos de legajo asignados a un DNI.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[{ "id": 1, "file_hash": "uuid...", "original_name": "doc.pdf", "tipo_documento": "Resolución", "fecha": "2024-01-15" }]
```

---

## Usuarios `/usuarios` 🔒 (solo administradores)

Todas las rutas requieren JWT con `nivel = 1`. Las demás devuelven `401`.

### `POST /usuarios/listar`

Lista todos los usuarios del sistema.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Administrador", "nickname": "admin", "nivel": 1 }]
```

---

### `POST /usuarios/crear`

Crear un nuevo usuario.

**Body:**

| Campo      | Tipo   | Requerido                        |
| ---------- | ------ | -------------------------------- |
| `nombre`   | string | sí                               |
| `nickname` | string | sí (mín. 3 caracteres)           |
| `pass`     | string | sí (mín. 4 caracteres)           |
| `nivel`    | number | sí (`1` = admin, `2` = usuario)  |

**Respuesta:** `"Usuario creado correctamente"`

---

### `POST /usuarios/editar`

Editar nombre, nickname y nivel de un usuario (no cambia contraseña).

**Body:**

| Campo      | Tipo   | Requerido |
| ---------- | ------ | --------- |
| `id`       | number | sí        |
| `nombre`   | string | sí        |
| `nickname` | string | sí        |
| `nivel`    | number | sí        |

**Respuesta:** `"Usuario actualizado correctamente"`

---

### `POST /usuarios/eliminar`

Eliminar un usuario. No se puede eliminar la propia cuenta.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `id`  | number | sí        |

**Respuesta:** `"Usuario eliminado"`

---

### `POST /usuarios/reset_pass`

Restablecer la contraseña de un usuario (sin verificar la actual).

**Body:**

| Campo        | Tipo   | Requerido              |
| ------------ | ------ | ---------------------- |
| `id`         | number | sí                     |
| `nueva_pass` | string | sí (mín. 4 caracteres) |

**Respuesta:** `"Contraseña restablecida correctamente"`
