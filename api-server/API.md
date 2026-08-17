# API Endpoints

Base URL: `http://127.0.0.1:4010`

Todas las rutas protegidas requieren el header `token` con un JWT válido.

## Errores

Todas las rutas devuelven errores con la misma estructura:

```json
{ "code": 400 | 401 | 404 | 500, "error": "Mensaje descriptivo" }
```

---

## Login y Registro `/login`

### `POST /login/`

Autenticación con Google OAuth.

**Body:**

```json
{ "google_sub": "1234567890", "email": "usuario@ejemplo.com" }
```

**Respuesta (Éxito):**

```json
{
  "token": "eyJhbGciOi...",
  "user": {
    "id": 1,
    "google_sub": "1234567890",
    "email": "usuario@ejemplo.com",
    "full_name": "Nombre Usuario",
    "picture_url": "https://lh3.googleusercontent.com/a/...",
    "role": "USER",
    "status": "APPROVED",
    "created_at": "2026-07-27T12:00:00",
    "updated_at": "2026-07-27T12:00:00"
  }
}
```

---

### `POST /login/register`

Registro de nuevo usuario con datos de Google OAuth.

**Body:**

```json
{
  "google_sub": "1234567890",
  "email": "usuario@ejemplo.com",
  "full_name": "Nombre Usuario",
  "picture_url": "https://lh3.googleusercontent.com/a/..."
}
```

**Respuesta:**

```json
{
  "id": 1,
  "message": "Registro completado con éxito. Tu cuenta está pendiente de aprobación por un administrador."
}
```

---

## Dashboard `/api/dash` 🔒

Todas las rutas de `/api/dash` requieren JWT.

### `GET /api/dash/cumpleanos`

Cumpleaños próximos (±5 días pasados / +30 días futuros).

**Respuesta:**

```json
[{ "dni": "12345678", "nombre": "Apellido Nombre", "nacimiento": "1990-01-15", "edad": 36, "avatar": "/personal/avatar/12345678", "regimen": "D.L. 276" }]
```

---

### `GET /api/dash/resumen`

Resumen general del personal.

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

### `GET /api/dash/areareport`

Personal activo por área.

**Respuesta:**

```json
[{ "cantidad": 10, "nombre": "Gerencia Municipal" }]
```

---

### `GET /api/dash/renunciasano`

Renuncias del año en curso por área.

**Respuesta:**

```json
[{ "cantidad": 3, "nombre": "Gerencia Municipal" }]
```

---

### `GET /api/dash/bancosreport`

Lista de bancos.

**Respuesta:**

```json
[{ "id": 1, "nombre": "BCP" }]
```

---

### `GET /api/dash/activos`

Reporte de personal activo completo.

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

### `GET /api/dash/activos/area?area_id=1` o `GET /api/dash/activos/area?area=Nombre`

Reporte de personal activo por ID o nombre de área.

**Query params:**

| Campo     | Tipo   | Requerido |
| --------- | ------ | --------- |
| `area_id` | number | opcional  |
| `area`    | string | opcional  |

**Respuesta:** Igual que `/activos` con campo extra `avatar`.

---

### `GET /api/dash/activos/regimen?regimen_id=1` o `GET /api/dash/activos/regimen?regimen=Nombre`

Reporte de personal activo por ID o nombre de régimen laboral.

**Query params:**

| Campo        | Tipo   | Requerido |
| ------------ | ------ | --------- |
| `regimen_id` | number | opcional  |
| `regimen`    | string | opcional  |

**Respuesta:** Igual que `/activos` con campo extra `avatar`.

---

### `GET /api/dash/activos/sindicato?sindicato_id=1` o `GET /api/dash/activos/sindicato?sindicato=Nombre`

Reporte de personal activo por ID o nombre de sindicato.

**Query params:**

| Campo          | Tipo   | Requerido |
| -------------- | ------ | --------- |
| `sindicato_id` | number | opcional  |
| `sindicato`    | string | opcional  |

**Respuesta:** Igual que `/activos` con campo extra `avatar`.

---

### `GET /api/dash/historial?dni=12345678&key=...`

Historial de operaciones filtrado por DNI.

**Query params:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |
| `key` | string | sí        |

**Respuesta:**

```json
[{ "operacion": "editar", "detalle": "...", "fecha": "2024-01-15 10:30:00", "nombre": "Admin" }]
```

---

### `GET /api/dash/organigrama`

Organigrama jerárquico.

**Respuesta:**

```json
[{ "id": 1, "area": "Gerencia Municipal", "jefe": "Apellido Nombre", "dni": "12345678", "subgerencias": [] }]
```

---

### `GET /api/dash/report-renuncia`

Renuncias recientes (últimos 120 días).

**Respuesta:**

```json
[{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", "fecha": "2024-01-15", "cargo": "Analista", "area": "Gerencia", "codigo": "P001", "avatar": "/personal/avatar/12345678" }]
```

---

### `GET /api/dash/documentos`

Lista de tipos de documento.

**Respuesta:**

```json
[{ "id": 1, "nombre": "Resolución", "sigla": "RA" }]
```

---

### `GET /api/dash/exportar_excel`

Genera y descarga un archivo Excel con el padrón de personal activo y pendiente.

**Respuesta:** `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` (binario)

---

### `GET /api/dash/activos/distrito`

Cantidad de personal activo agrupado por distrito de residencia.

**Respuesta:**

```json
[{ "distrito": "PIURA", "cantidad": 45 }, { "distrito": "SIN ASIGNAR", "cantidad": 5 }]
```

---

### `GET /api/dash/trabajadores_nuevos`

Trabajadores ingresados en los últimos 120 días.

**Respuesta:**

```json
[{ "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", "ingreso": "2024-01-15", "documento": "RA-001-2024", "area": "Gerencia", "cargo": "Analista", "regimen": "D.L. 276", "sueldo": 2500.0, "plaza": "P001", "avatar": "/personal/avatar/12345678" }]
```

---

### `GET /api/dash/rangos_edad`

Distribución de personal por rangos de edad.

**Respuesta:**

```json
[{ "cantidad": 15, "nombre": "25-35" }]
```

---

### `GET /api/dash/rangos_antiguedad`

Distribución de personal por rangos de antigüedad.

**Respuesta:**

```json
[{ "cantidad": 20, "nombre": "1-5 años" }]
```

---

### `GET /api/dash/reporte_eventos`

Todos los eventos de vínculo registrados (rotaciones, abandonos).

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
    "descripcion_salida": null,
    "avatar": "/personal/avatar/12345678"
  }
]
```

---

### `POST /api/dash/comparar_mef`

Compara dos archivos Excel (CAS y Otros) del MEF.

**Body (multipart/form-data):**

| Campo        | Tipo | Requerido |
| ------------ | ---- | --------- |
| `file_cas`   | file | sí        |
| `file_otros` | file | sí        |

**Respuesta:**

```json
{ "comparaciones": [...] }
```

---

### `POST /api/dash/generar_mef`

Genera un Excel de comparación MEF a partir de un JSON.

**Body:**

```json
{ "comparaciones": [...] }
```

**Respuesta:** `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` (binario)

---

### `GET /api/dash/alerta_70`

Alerta de servidores activos que van a cumplir o han cumplido 70 años de edad (límite de cese legal laboral). Calcula fecha de cese regular (último día del mes de cumpleaños) y fecha de cese extendido (31 de diciembre del año en curso).

**Query Params (Opcionales):**

| Parámetro  | Tipo      | Requerido | Descripción                                              |
| ---------- | --------- | --------- | -------------------------------------------------------- |
| `edad_min` | `integer` | no        | Edad mínima a filtrar (por defecto `69` para preventivo) |

**Respuesta:**

```json
[
  {
    "dni": "01234567",
    "nombre": "PEREZ LOPEZ JUAN",
    "nacimiento": "1956-08-20",
    "edad_actual": 69,
    "fecha_70_anos": "2026-08-20",
    "fecha_limite_mes": "2026-08-31",
    "fecha_extension_fin_ano": "2026-12-31",
    "dias_para_70": 4,
    "dias_para_cese_mes": 15,
    "dias_para_cese_extension": 137,
    "estado_alerta": "CUMPLE_ESTE_MES",
    "area": "SUBGERENCIA DE RECURSOS HUMANOS",
    "cargo": "ESPECIALISTA ADMINISTRATIVO",
    "regimen": "D.L. 276",
    "plaza": "P-045",
    "avatar": "/personal/avatar/01234567"
  }
]
```

---

## Personal `/personal` 🔒


Todas las rutas de `/personal` requieren JWT **excepto `GET /personal/avatar/{dni}`**.

### `GET /personal/avatar/{dni}`

Obtener la imagen de avatar de un trabajador. No requiere JWT.

**Respuesta:** Imagen PNG (`Content-Disposition: inline`).

**Errores:** `400` si el DNI es inválido, `404` si no existe el avatar.

---

### `POST /personal/avatar` 🔒

Subir o actualizar avatar de un trabajador (base64).

**Body:**

```json
{ "dni": "12345678", "imagen_base64": "iVBORw0KGgo..." }
```

**Validaciones:** DNI de 8 dígitos, imagen base64 máx. 3 MB.

**Respuesta:**

```json
{ "avatar": "/personal/avatar/12345678" }
```

---

### `GET /personal/perfil/{dni}`

Perfil de un trabajador.

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

### `GET /personal/buscar?nombre=Juan`

Buscar trabajadores por nombre.

**Query params:**

| Campo    | Tipo   | Requerido |
| -------- | ------ | --------- |
| `nombre` | string | sí        |

**Respuesta:**

```json
[{ "nombre": "Nombre Apellido", "dni": "12345678", "estado": "activo", "sexo": "M" }]
```

---

### `PUT /personal/editar_por_dni`

Editar datos personales.

**Body:**

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

**Respuesta:** `"Rows affected: 1"`

---

### `GET /personal/consultar_dni/{dni}`

Consultar datos por DNI. Busca localmente; si no existe, consulta la API de RENIEC.

**Respuesta:**

```json
{ "dni": "12345678", "apaterno": "GARCIA", "amaterno": "LOPEZ", "nombre": "JUAN", "nacimiento": "1990-01-15", "sexo": "M" }
```

---

### `GET /personal/banco/{dni}`

Datos bancarios de un trabajador.

**Respuesta:**

```json
{ "id": 1, "numero_cuenta": "123456789", "tipo_cuenta": "AHORRO", "cci": "00212345678901234567", "banco": "BCP", "estado": 1 }
```

---

### `POST /personal/agregar_infobancaria`

Agregar cuenta bancaria.

**Body:**

```json
{ "numero_cuenta": "123456789", "tipo_cuenta": "AHORRO", "cci": "002...", "banco": 1, "estado": 1, "dni": "12345678" }
```

**Respuesta:** `"Rows affected: 1"`

---

### `PUT /personal/editar_infobancaria`

Editar cuenta bancaria existente.

**Body:**

```json
{ "id": 1, "numero_cuenta": "123456789", "tipo_cuenta": "AHORRO", "cci": "002...", "banco": "BCP", "estado": 1, "dni": "12345678" }
```

**Respuesta:** `"Rows affected: 1"`

---

### `GET /personal/grado/{dni}`

Grados académicos de un trabajador.

**Respuesta:**

```json
[{ "id": 1, "profesion": "ABOGADO", "universidad": "UNIV.", "nivel_academico": "TITULADO", "abrv": "ABOG", "dni": "12345678", "fecha": "2020-01-15" }]
```

---

### `POST /personal/agregar_gradoa`

Agregar o actualizar grado académico (upsert).

**Body:**

```json
{ "id": 0, "profesion": "ABOGADO", "universidad": "UNIV.", "nivel_academico": "TITULADO", "abrv": "ABOG", "dni": "12345678", "fecha": "2020-01-15" }
```

> `id = 0` para insertar, `id > 0` para editar.

**Respuesta:** `"Operación exitosa. Filas afectadas: 1"`

---

### `DELETE /personal/eliminar_gradoa/{id}`

Eliminar un grado académico por ID.

**Respuesta:** `"Grado académico eliminado"`

**Errores:** `404` si el ID no existe.

---

### `GET /personal/contacto/{dni}`

Contacto de emergencia de un trabajador.

**Respuesta:**

```json
{ "persona_dni": "12345678", "nombre": "María", "relacion": "Madre", "telefono": "999999999" }
```

---

### `POST /personal/agregar_contacto`

Agregar o actualizar contacto de emergencia (upsert por DNI).

**Body:**

```json
{ "persona_dni": "12345678", "nombre": "María", "relacion": "Madre", "telefono": "999999999" }
```

**Respuesta:** `"Rows affected: 1"`

---

### `DELETE /personal/eliminar_contacto/{id}`

Eliminar el contacto de emergencia de un trabajador.

**Respuesta:** `"Contacto de emergencia eliminado"`

---

### `POST /personal/agregar_sindicato`

Afiliar vínculos a un sindicato.

**Body:**

```json
{
  "tipoDocumento": "RA",
  "numeroDocumento": 123,
  "añoDocumento": 2024,
  "fecha": "2024-01-15",
  "fechaValida": null,
  "descripcion": "Afiliación",
  "sindicato": 1,
  "vinculos": [{ "id_vinculo": 1, "dni": "12345678" }]
}
```

**Respuesta:** `"Se registraron correctamente los datos"`

---

### `DELETE /personal/eliminar_sindicato/{id}`

Desafiliar un vínculo de su sindicato.

**Path:** `{id}` = ID del vínculo.

**Body:**

```json
{
  "vinculo_id": 1,
  "dni": "12345678",
  "tipoDocumento": "RA",
  "numeroDocumento": 123,
  "añoDocumento": 2024,
  "fecha": "2024-01-15",
  "fechaValida": null,
  "descripcion": "Desafiliación"
}
```

**Respuesta:** `"Afiliación sindical eliminada"`

---

### `PUT /personal/editar_documento`

Editar un documento existente.

**Body:**

```json
{
  "dni": "12345678",
  "documento": {
    "id": 10,
    "tipoDocumento": "RA",
    "numeroDocumento": 123,
    "añoDocumento": 2024,
    "fecha": "2024-01-15",
    "fechaValida": null,
    "descripcion": "Descripción del documento"
  }
}
```

**Respuesta:** `"Documento actualizado"`

---

### `GET /personal/vinculos/{dni}`

Vínculos laborales de un trabajador.

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

### `POST /personal/registrar_trabajador`

Registrar un nuevo trabajador con persona, documento y vínculo.

**Body:**

```json
{
  "personal": {
    "dni": "12345678",
    "amaterno": "LOPEZ",
    "apaterno": "GARCIA",
    "nombre": "JUAN",
    "telf": "999999999",
    "direccion": "Av. Ejemplo 123",
    "email": "correo@mail.com",
    "ruc": null,
    "nacimiento": "1990-01-15",
    "sexo": "M",
    "region": "PIURA",
    "distrito": "PIURA"
  },
  "airshp": "P001",
  "documento": {
    "tipoDocumento": "RA",
    "numeroDocumento": 123,
    "añoDocumento": 2024,
    "fecha": "2024-01-15",
    "fechaValida": null,
    "descripcion": "Contratación"
  },
  "regimen": 1,
  "cargo": 2,
  "area": 3,
  "sueldo": 2500.0
}
```

**Respuesta:** `"Trabajador registrado correctamente"`

---

### `DELETE /personal/eliminar_vinculo/{id}`

Eliminar un vínculo laboral y sus asociaciones (sindicato, documentos).

**Respuesta:** `"Vínculo eliminado correctamente"`

---

### `POST /personal/renuncia_por_vinculo`

Registrar renuncia de un vínculo. El campo `id` del body es el ID del vínculo.

**Body:**

```json
{
  "id": 1,
  "tipoDocumento": "RA",
  "numeroDocumento": 456,
  "añoDocumento": 2024,
  "fecha": "2024-06-15",
  "fechaValida": null,
  "descripcion": "Renuncia voluntaria"
}
```

**Respuesta:**

```json
{
  "dni": "12345678",
  "nombre": "Apellido Nombre",
  "estado": "inactivo",
  "fecha": "2024-06-15",
  "descripcion": "Renuncia voluntaria",
  "documento": "RA-456-2024-RA"
}
```

---

### `PUT /personal/upsert_evento_vinculo`

Agregar o cerrar un evento de vínculo (rotación, abandono).

- Si se envía `documento_inicio`: crea un evento nuevo.
- Si se envía `documento_salida` + `id` del evento: cierra el evento existente.

**Body (crear evento):**

```json
{
  "id": null,
  "vinculo_id": 1,
  "tipo_evento": "rotacion",
  "nueva_area_id": 5,
  "documento_inicio": {
    "tipoDocumento": "RA",
    "numeroDocumento": 789,
    "añoDocumento": 2024,
    "fecha": "2024-03-01",
    "fechaValida": null,
    "descripcion": "Rotación de área"
  },
  "documento_salida": null,
  "estado": null
}
```

**Body (cerrar evento):**

```json
{
  "id": 10,
  "vinculo_id": 1,
  "tipo_evento": "rotacion",
  "nueva_area_id": null,
  "documento_inicio": null,
  "documento_salida": {
    "tipoDocumento": "RA",
    "numeroDocumento": 999,
    "añoDocumento": 2024,
    "fecha": "2024-06-01",
    "fechaValida": null,
    "descripcion": "Cierre de rotación"
  },
  "estado": null
}
```

**Respuesta:** `"Operación exitosa"`

---

### `DELETE /personal/delete_evento_vinculo/{id}`

Eliminar un evento de vínculo y sus documentos asociados.

**Respuesta:** `"Evento de vínculo eliminado"`

---

### `POST /personal/cambio_area`

Registra un **cambio definitivo** de área de un vínculo. Distinto a una rotación: actualiza `vinculo.area_id` de forma permanente y guarda el historial en la tabla `cambio_area`.

**Reglas:**

- El vínculo debe existir y estar `activo`.
- La nueva área debe existir y ser distinta a la actual.
- El vínculo **no debe tener una rotación activa** en `eventovinculo`. Si la tiene, devuelve `400` y debe cerrarse primero.
- El `user_id` que registra se toma del JWT.

**Body:**

```json
{
  "vinculo_id": 1,
  "nueva_area_id": 5,
  "fecha_cambio": "2024-03-01",
  "documento": {
    "tipoDocumento": "RA",
    "numeroDocumento": 789,
    "añoDocumento": 2024,
    "fecha": "2024-03-01",
    "fechaValida": null,
    "descripcion": "Cambio de área"
  }
}
```

**Respuesta:** `"Cambio de área registrado correctamente"`

**Errores:**

- `404` si el vínculo o la nueva área no existen.
- `400` si el vínculo no está activo, ya pertenece a esa área, o tiene una rotación activa.

---

### `GET /personal/buscar_vacantes`

Vacantes recientes (plazas con estado `vacante`).

**Respuesta:**

```json
[{ "id": 1, "dni": null, "nombre": null, "fecha": "2024-06-15", "area": "Gerencia", "cargo": "Analista", "codigo": "P001", "sueldo": 2500.0, "avatar": null }]
```

---

### `GET /personal/buscar_por_plaza?codigo=P001`

Detalle de una plaza por código.

**Query params:**

| Campo    | Tipo   | Requerido |
| -------- | ------ | --------- |
| `codigo` | string | sí        |

**Respuesta:**

```json
{
  "codigo": "P001",
  "cargo_estructural": "CE001",
  "cargo_descripcion": "Analista",
  "grupo_ocupacional": "G1",
  "grupo_descripcion": "Grupo 1",
  "condicion": "Nombrado",
  "regimen_id": 1,
  "regimen": "D.L. 276"
}
```

---

### `GET /personal/buscar_areas`

Lista de áreas activas.

**Respuesta:**

```json
[{ "id": 1, "nombre": "Gerencia Municipal", "activo": true, "nivel": 1 }]
```

---

### `GET /personal/buscar_cargos`

Lista de cargos activos.

**Respuesta:**

```json
[{ "id": 1, "nombre": "Analista", "activo": true }]
```

---

### `GET /personal/activos_por_distrito?distrito=PIURA`

Detalle y estadísticas de trabajadores **activos** que residen en un distrito específico. Coincidencia exacta y *case-insensitive*. Para listar quienes no tienen distrito asignado, usar `"SIN ASIGNAR"`. Incluye conteo por áreas con su ID, rangos de edad, y la lista completa de personas con IDs de área, cargo y régimen.

**Query params:**

| Campo      | Tipo   | Requerido |
| ---------- | ------ | --------- |
| `distrito` | string | sí        |

**Respuesta:**

```json
{
  "distrito": "PIURA",
  "total": 1,
  "areas": [
    {
      "id": 1,
      "nombre": "Gerencia",
      "cantidad": 1
    }
  ],
  "rangos_edad": [
    {
      "nombre": "26-35",
      "cantidad": 1
    }
  ],
  "personas": [
    {
      "dni": "12345678",
      "nombre": "García López Juan",
      "ingreso": "2020-01-15",
      "direccion": "Av. Ejemplo 123",
      "area": {
        "id": 1,
        "nombre": "Gerencia"
      },
      "cargo": {
        "id": 5,
        "nombre": "Analista"
      },
      "regimen": {
        "id": 2,
        "nombre": "D.L. 276"
      },
      "sindicato": null,
      "distrito": "PIURA",
      "avatar": null
    }
  ]
}
```

---

### `GET /personal/calidad_datos`

Reporte de calidad de datos: trabajadores activos sin domicilio o sin documento de salida.

**Respuesta:**

```json
{
  "sin_domicilio": [
    { "dni": "12345678", "nombre": "Apellido Nombre", "direccion": null, "distrito": null }
  ],
  "sin_documento_salida": [
    { "id": 1, "dni": "12345678", "nombre": "Apellido Nombre", "cargo": "Analista", "area": "Gerencia" }
  ]
}
```

---

## Fileserver `/fileserver`

Las rutas requieren JWT **excepto `GET /fileserver/{hash}`**.

### `GET /fileserver/{hash}`

Acceder a un archivo por su hash. No requiere JWT (se puede usar en `<img>` o visor de PDF).

**Respuesta:**

- Si el registro tiene archivo local (`file_hash` en disco): contenido binario del archivo (`Content-Disposition: inline`).
- Si el registro tiene `external_url` (registrado vía `/fileserver/registrar_url`): redirección **HTTP 302** al `external_url`.
- `404` si el hash no existe en la BD o el archivo local no está en disco.

---

### `POST /fileserver/upload` 🔒

Subir un archivo PDF (máx. 10 MB).

**Body (multipart/form-data):**

| Campo          | Tipo   | Requerido |
| -------------- | ------ | --------- |
| `archivo/file` | file   | sí        |
| `dni_asociado` | string | sí        |
| `documento_id` | number | no        |

**Validación:** Solo se permiten archivos PDF.

**Respuesta:**

```json
[{ "id": 1, "original_name": "mi_doc.pdf", "file_hash": "uuid...", "extension": "pdf" }]
```

---

### `POST /fileserver/upload_batch` 🔒

Sube un archivo PDF (máx. 20 MB) y lo vincula a múltiples trabajadores creando un documento único.

**Body (multipart/form-data):**

| Campo               | Tipo    | Requerido |
| ------------------- | ------- | --------- |
| `archivo/file`      | file    | sí        |
| `tipo_documento_id` | number  | sí        |
| `numero`            | string  | sí        |
| `year`              | number  | sí        |
| `fecha`             | string  | sí        |
| `fecha_valida`      | string? | no        |
| `descripcion`       | string  | sí        |
| `nombre_archivo`    | string? | no        |
| `dnis`              | string  | sí (lista separada por comas o JSON array) |

**Respuesta:**

```json
{
  "message": "Documento registrado y vinculado correctamente a todos los trabajadores",
  "documento_id": 123,
  "file_hash": "uuid...",
  "count": 5
}
```

---

### `POST /fileserver/registrar_url` 🔒

Registra un PDF que vive en una URL externa (portal de transparencia, Drive, otro servidor) sin subirlo al disco. La fila resultante convive en la misma tabla `fileserver` que los archivos locales: tendrá `external_url` poblado y un `file_hash` autogenerado que sirve como identificador uniforme para `GET /fileserver/{hash}` (que responderá con redirect 302).

**Body (application/json):**

```json
{
  "dni_asociado": "12345678",
  "original_name": "RA-123-2026.pdf",
  "external_url": "https://transparencia.munives.gob.pe/docs/RA-123-2026.pdf",
  "documento_id": null
}
```

**Validaciones:**

- `external_url` debe comenzar con `https://`.
- `original_name` debe terminar en `.pdf`.
- `dni_asociado` no puede estar vacío.

**Respuesta:**

```json
{
  "id": 124,
  "original_name": "RA-123-2026.pdf",
  "file_hash": "9f8e7d6c-...",
  "external_url": "https://transparencia.munives.gob.pe/docs/RA-123-2026.pdf",
  "extension": "pdf"
}
```

---

### `GET /fileserver/archivos_por_dni/{dni}` 🔒

Lista de archivos de un DNI.

**Respuesta:**

```json
[{ "id": 1, "documento_id": 10, "dni_asociado": "12345678", "original_name": "mi_doc.pdf", "file_hash": "uuid...", "extension": "pdf", "external_url": null, "usuario_subida": "Admin", "fecha_subida": "2024-01-15 10:30:00" }]
```

---

### `DELETE /fileserver/eliminar_archivo/{id}` 🔒

Eliminar un archivo por ID. Borra el archivo del disco si es local; solo elimina el registro si tiene `external_url`.

**Respuesta:** `"Archivo eliminado correctamente"`

**Errores:** `404` si el archivo no existe.

---

### `POST /fileserver/asignar_documento` 🔒

Asignar un archivo a un documento de legajo.

**Body:**

```json
{ "id": 1, "documento_id": 10 }
```

**Respuesta:**

```json
{ "message": "Documento asignado correctamente" }
```

---

### `GET /fileserver/documentos/{dni}` 🔒

Lista de documentos de legajo asignados a un DNI (ingreso y salida).

**Respuesta:**

```json
[{ "id": 1, "sigla": "Resolución N° 123-2024-RA", "fecha": "2024-01-15", "descripcion": "Descripción" }]
```

---

### `PUT /fileserver/renombrar_archivo` 🔒

Renombrar el `original_name` de un archivo.

**Body:**

```json
{ "id": 1, "nuevo_nombre": "RA-123-2024" }
```

> Si el nombre no termina en `.pdf`, se le añade automáticamente.

**Respuesta:**

```json
{ "message": "Nombre actualizado correctamente", "id": 1, "original_name": "RA-123-2024.pdf" }
```

**Errores:** `404` si el archivo no existe, `400` si el nombre está vacío.

---

## Usuarios `/usuarios` 🔒 (solo administradores)

Todas las rutas requieren JWT con `role = "ADMIN"`. Las demás devuelven `401`.

### `GET /usuarios/listar`

Lista todos los usuarios del sistema.

**Respuesta:**

```json
[
  {
    "id": 1,
    "google_sub": "1234567890",
    "email": "admin@ejemplo.com",
    "full_name": "Administrador",
    "picture_url": null,
    "role": "ADMIN",
    "status": "APPROVED",
    "created_at": "2026-07-27T12:00:00",
    "updated_at": "2026-07-27T12:00:00"
  }
]
```

---

### `POST /usuarios/crear`

Crear un nuevo usuario desde el panel administrativo.

**Body:**

```json
{
  "google_sub": "1234567890",
  "email": "usuario@ejemplo.com",
  "full_name": "Nombre Usuario",
  "picture_url": null
}
```

**Respuesta:** `"Usuario creado correctamente"`

---

### `PUT /usuarios/editar`

Editar el rol (`ADMIN` / `USER`) y el estado (`PENDING` / `APPROVED` / `REJECTED`) de un usuario.

**Body:**

```json
{ "id": 1, "role": "ADMIN", "status": "APPROVED" }
```

**Respuesta:** `"Usuario actualizado correctamente"`

---

### `DELETE /usuarios/eliminar/{id}`

Eliminar un usuario. No se puede eliminar la propia cuenta.

**Errores:** `400` si se intenta eliminar la propia cuenta.

**Respuesta:** `"Usuario eliminado"`

---

## Documentos `/personal`

### `GET /personal/documento/{id}` 🔒

Obtener un documento por su ID.

**Parámetros:**
- `id` (path): ID del documento.

**Respuesta (Éxito):**

```json
{
  "id": 1,
  "tipoDocumento": "1",
  "areaId": 5,
  "numeroDocumento": 123,
  "añoDocumento": 2026,
  "fecha": "2026-01-15",
  "fechaValida": "2026-12-31",
  "conv": null,
  "descripcion": "Resolución de nombramiento",
  "funcion": null
}
```

---

### `POST /personal/crear_documento` (o `POST /personal/documento`) 🔒

Crear un nuevo documento.

**Body:**

```json
{
  "dni": "12345678",
  "documento": {
    "tipoDocumento": "1",
    "areaId": 5,
    "numeroDocumento": 123,
    "añoDocumento": 2026,
    "fecha": "2026-01-15",
    "fechaValida": "2026-12-31",
    "descripcion": "Resolución de nombramiento"
  }
}
```

**Respuesta (201 Created):**

```json
{
  "message": "Documento creado correctamente",
  "id": 1
}
```

---

### `PUT /personal/editar_documento` 🔒

Editar un documento existente.

**Body:**

```json
{
  "dni": "12345678",
  "documento": {
    "id": 1,
    "tipoDocumento": "1",
    "areaId": 5,
    "numeroDocumento": 124,
    "añoDocumento": 2026,
    "fecha": "2026-01-15",
    "fechaValida": "2026-12-31",
    "descripcion": "Resolución corregida"
  }
}
```

**Respuesta:** `"Documento actualizado"`

---

### `DELETE /personal/eliminar_documento/{id}` (o `DELETE /personal/documento/{id}`) 🔒

Eliminar un documento por su ID.

**Parámetros:**
- `id` (path): ID del documento a eliminar.

**Respuesta:** `"Documento eliminado correctamente"`

