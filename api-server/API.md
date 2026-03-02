# API Endpoints

Base URL: `http://127.0.0.1:8080`

Todas las rutas protegidas requieren el header `token` con un JWT válido.

## Errores

Todas las rutas devuelven errores con la misma estructura:

```json
{ "code": 400 | 401 | 404 | 500, "error": "Mensaje descriptivo" }
```

> Los errores `400` de validación incluyen el campo y el motivo, por ejemplo:
> `{ "code": 400, "error": "dni: Debe tener exactamente 8 dígitos numéricos" }`

---

## Login `/login`

### `POST /login/`

Autenticación de usuario.

**Body:**

| Campo      | Tipo   | Requerido | Validación                |
| ---------- | ------ | --------- | ------------------------- |
| `username` | string | sí        | No puede estar vacío      |
| `password` | string | sí        | No puede estar vacío      |

**Respuesta:**

```json
{ "token": "eyJhbGciOi..." }
```

---

### `POST /login/changepass` 🔒

Cambiar contraseña.

**Body:**

| Campo     | Tipo   | Requerido | Validación                              |
| --------- | ------ | --------- | --------------------------------------- |
| `id`      | number | sí        | > 0                                     |
| `oldpass` | string | sí        | No puede estar vacía                    |
| `newpass` | string | sí        | Mínimo 4 caracteres                     |

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
  {
    "dni": "12345678",
    "nombre": "Apellido Nombre",
    "nacimiento": "1990-01-15",
    "edad": 36
  }
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
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[
  {
    "operacion": "editar",
    "detalle": "...",
    "fecha": "2024-01-15 10:30:00",
    "nombre": "Admin"
  }
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
    "id": 1,
    "nombre": "Apellido Nombre",
    "dni": "12345678",
    "estado": "prestamo",
    "persona": "Juan",
    "userid": 1,
    "usuario": "Admin",
    "fecha": "2024-01-15T10:30:00"
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
    "id": 1,
    "dni": "12345678",
    "nombre": "Apellido Nombre",
    "fecha": "2024-01-15",
    "cargo": "Analista",
    "area": "Gerencia",
    "codigo": "P001"
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

| Campo    | Tipo   | Requerido | Validación             |
| -------- | ------ | --------- | ---------------------- |
| `nombre` | string | sí        | No puede estar vacío   |

**Respuesta:**

```json
[
  {
    "nombre": "Nombre Apellido",
    "dni": "12345678",
    "estado": "activo",
    "sexo": "M"
  }
]
```

---

### `POST /personal/por_dni`

Perfil de un trabajador.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

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
  "sexo": "M"
}
```

---

### `POST /personal/editar_por_dni`

Editar datos personales.

**Body:**

| Campo        | Tipo    | Requerido | Validación                      |
| ------------ | ------- | --------- | ------------------------------- |
| `dni`        | string  | sí        | 8 dígitos numéricos exactamente |
| `nombre`     | string? | no        |                                 |
| `telf`       | string? | no        |                                 |
| `direccion`  | string? | no        |                                 |
| `email`      | string? | no        |                                 |
| `ruc`        | string? | no        |                                 |
| `nacimiento` | string  | sí        | Formato `YYYY-MM-DD`            |
| `sexo`       | string? | no        |                                 |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/vinculos_por_dni`

Vínculos laborales de un trabajador.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
[
  {
    "id": 1,
    "dni": "12345678",
    "doc_ingreso": "RA",
    "numero_doc_ingreso": "001",
    "descrip_ingreso": "...",
    "fecha_ingreso": "2020-01-15",
    "area": "Gerencia",
    "cargo": "Analista",
    "regimen": "D.L. 276",
    "sueldo": 2500.0,
    "codigo": "P001",
    "cargo_estructural": "...",
    "grupo_ocupacional": "...",
    "estado": "activo",
    "doc_salida": null,
    "descrip_salida": null,
    "fecha_salida": null,
    "numero_doc_salida": null,
    "sindicato": null,
    "tipo_evento": null,
    "estado_evento": null,
    "doc_evento_tipo": null,
    "numero_doc_evento": null,
    "fecha_evento": null
  }
]
```

---

### `POST /personal/renuncia_por_vinculo`

Registrar renuncia de un vínculo.

**Body:**

| Campo             | Tipo    | Requerido       | Validación                     |
| ----------------- | ------- | --------------- | ------------------------------ |
| `id`              | number? | sí (vinculo id) |                                |
| `tipoDocumento`   | string? | no              |                                |
| `numeroDocumento` | number? | no              |                                |
| `añoDocumento`    | number? | no              |                                |
| `fecha`           | string  | sí              | No puede estar vacía           |
| `fechaValida`     | string? | no              |                                |
| `conv`            | number? | no              |                                |
| `descripcion`     | string  | sí              | No puede estar vacía           |
| `funcion`         | number? | no              |                                |
| `sueldo`          | number? | no              |                                |

**Respuesta:**

```json
{
  "dni": "12345678",
  "nombre": "Analista",
  "estado": "inactivo",
  "fecha": "2024-06-15",
  "descripcion": "Renuncia voluntaria",
  "documento": "RA-001-2024"
}
```

---

### `POST /personal/banco_por_dni`

Datos bancarios de un trabajador.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
{
  "id": 1,
  "numero_cuenta": "123456789",
  "tipo_cuenta": "AHORRO",
  "cci": "00212345678901234567",
  "banco": "BCP",
  "estado": 1,
  "dni": "12345678"
}
```

---

### `POST /personal/agregar_infobancaria`

Agregar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido | Validación                       |
| --------------- | ------- | --------- | -------------------------------- |
| `numero_cuenta` | string  | sí        | 6-30 caracteres                  |
| `tipo_cuenta`   | string  | sí        | No puede estar vacío             |
| `cci`           | string? | no        |                                  |
| `banco`         | number  | sí (id)   |                                  |
| `estado`        | number  | sí        |                                  |
| `dni`           | string  | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/editar_infobancaria`

Editar cuenta bancaria.

**Body:**

| Campo           | Tipo    | Requerido | Validación                       |
| --------------- | ------- | --------- | -------------------------------- |
| `id`            | number  | sí        |                                  |
| `numero_cuenta` | string  | sí        | 6-30 caracteres                  |
| `tipo_cuenta`   | string? | no        |                                  |
| `cci`           | string? | no        |                                  |
| `banco`         | string  | sí        |                                  |
| `estado`        | number  | sí        |                                  |
| `dni`           | string  | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/grado_por_dni`

Grados académicos de un trabajador.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
[
  {
    "id": 15,
    "profesion": "ABOGADO",
    "universidad": "UNIVERSIDAD DE PIURA",
    "colegiatura": null,
    "nivel_academico": "TITULADO",
    "abrv": "ABOG",
    "dni": "41662616"
  },
  {
    "id": 16,
    "profesion": "CONTADORA",
    "universidad": "UNIVERSIDAD PRIVADA DEL NORTE",
    "colegiatura": null,
    "nivel_academico": "BACHILLER",
    "abrv": "BACH",
    "dni": "41662616"
  }
]
```

---

### `POST /personal/agregar_gradoa`

Agregar o actualizar grado académico (upsert). Si `id` es `0` se inserta, si `id > 0` se actualiza.

**Body:**

| Campo             | Tipo    | Requerido | Validación                       |
| ----------------- | ------- | --------- | -------------------------------- |
| `id`              | number  | sí        |                                  |
| `profesion`       | string  | sí        | Mínimo 2 caracteres              |
| `universidad`     | string  | sí        | Mínimo 2 caracteres              |
| `colegiatura`     | string? | no        |                                  |
| `nivel_academico` | string  | sí        | Mínimo 2 caracteres              |
| `abrv`            | string  | sí        | No puede estar vacío             |
| `dni`             | string  | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
"Operación exitosa. Filas afectadas: 1"
```

---

### `POST /personal/agregar_sindicato`

Afiliar vínculos a sindicato.

**Body:**

| Campo             | Tipo    | Requerido | Validación                    |
| ----------------- | ------- | --------- | ----------------------------- |
| `id`              | number? | no        |                               |
| `tipoDocumento`   | string? | no        |                               |
| `numeroDocumento` | number? | no        |                               |
| `añoDocumento`    | number? | no        |                               |
| `fecha`           | string  | sí        | No puede estar vacía          |
| `fechaValida`     | string? | no        |                               |
| `descripcion`     | string  | sí        | No puede estar vacía          |
| `sindicato`       | number  | sí (id)   |                               |
| `vinculos`        | array   | sí        |                               |

**Estructura de `vinculos`:**

| Campo        | Tipo   | Requerido |
| ------------ | ------ | --------- |
| `id_vinculo` | number | sí        |
| `dni`        | string | sí        |

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

| Campo | Tipo   | Requerido | Validación                      |
| ----- | ------ | --------- | ------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente |

**Respuesta:**

```json
[
  {
    "id": 1,
    "persona": "Juan Pérez",
    "dni": "12345678",
    "fecha": "2024-01-15",
    "estado": "prestamo",
    "descrip": "Legajo completo",
    "nuevo": 0,
    "user": 1
  }
]
```

---

### `POST /personal/nuevo_evento_legajo`

Agregar evento de legajo.

**Body:**

| Campo     | Tipo    | Requerido             | Validación              |
| --------- | ------- | --------------------- | ----------------------- |
| `id`      | number  | sí                    |                         |
| `persona` | string  | sí                    | Mínimo 2 caracteres     |
| `dni`     | string  | sí                    |                         |
| `fecha`   | string? | no                    |                         |
| `estado`  | string? | no                    |                         |
| `descrip` | string? | no                    |                         |
| `nuevo`   | number  | sí (1 = nuevo estado) |                         |
| `user`    | number? | no                    |                         |

**Respuesta:**

```json
"Se registraron correctamente los datos"
```

---

### `POST /personal/asistencia`

Reporte de asistencia mensual.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |
| `mes` | number | sí        | Entre 1 y 12                     |
| `año` | number | sí        | Entre 2000 y 2100                |

**Respuesta:**

```json
[{ "dni": "12345678", "hora": "08:00", "fecha": "2024-01-15" }]
```

---

### `POST /personal/agregar_contacto`

Agregar o actualizar contacto de emergencia.

**Body:**

| Campo         | Tipo    | Requerido | Validación                       |
| ------------- | ------- | --------- | -------------------------------- |
| `persona_dni` | string  | sí        | 8 dígitos numéricos exactamente  |
| `nombre`      | string  | sí        | Mínimo 2 caracteres              |
| `relacion`    | string  | sí        | Mínimo 2 caracteres              |
| `telefono`    | string? | no        |                                  |

**Respuesta:**

```json
"Rows affected: 1"
```

---

### `POST /personal/contacto_dni`

Contacto de emergencia de un trabajador.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
{
  "persona_dni": "12345678",
  "nombre": "María",
  "relacion": "Madre",
  "telefono": "999999999"
}
```

---

### `POST /personal/buscar_vacantes`

Vacantes recientes (últimos 3 meses).

**Body:** vacío

**Respuesta:**

```json
[
  {
    "id": 1,
    "dni": "12345678",
    "nombre": "Apellido Nombre",
    "fecha": "2024-06-15",
    "fechavalida": "2024-06-20",
    "area": "Gerencia",
    "cargo": "Analista",
    "codigo": "P001",
    "sueldo": 2500.0
  }
]
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
{
  "codigo": "P001",
  "cargo_estructural": "CE001",
  "cargo_descripcion": "Analista",
  "grupo_ocupacional": "GO001",
  "grupo_descripcion": "Profesional",
  "condicion": "Nombrado",
  "regimen_id": 1,
  "regimen": "D.L. 276"
}
```

---

### `POST /personal/registrar_trabajador`

Registrar un nuevo trabajador con persona, documento y vínculo.

**Body:**

| Campo       | Tipo   | Requerido         | Validación                       |
| ----------- | ------ | ----------------- | -------------------------------- |
| `personal`  | object | sí                | Ver estructura abajo             |
| `airshp`    | string | sí (plaza código) | No puede estar vacío             |
| `documento` | object | sí                | Ver estructura abajo             |
| `regimen`   | number | sí (id)           | > 0                              |
| `cargo`     | number | sí (id)           | > 0                              |
| `area`      | number | sí (id)           | > 0                              |
| `sueldo`    | number | sí                | ≥ 0                              |

**Estructura de `personal`:**

| Campo        | Tipo    | Requerido | Validación                       |
| ------------ | ------- | --------- | -------------------------------- |
| `dni`        | string  | sí        | 8 dígitos numéricos exactamente  |
| `amaterno`   | string  | sí        | Mínimo 1 carácter                |
| `apaterno`   | string  | sí        | Mínimo 1 carácter                |
| `nombre`     | string  | sí        | Mínimo 1 carácter                |
| `telf`       | string? | no        |                                  |
| `direccion`  | string? | no        |                                  |
| `email`      | string? | no        |                                  |
| `ruc`        | string? | no        |                                  |
| `nacimiento` | string  | sí        | Formato `YYYY-MM-DD`             |
| `sexo`       | string? | no        |                                  |

**Respuesta:**

```json
"Trabajador registrado correctamente"
```

---

### `POST /personal/consultar_dni`

Consultar datos de una persona por DNI. Primero busca en la base de datos local; si no existe, consulta la API de RENIEC.

**Body:**

| Campo | Tipo   | Requerido | Validación                       |
| ----- | ------ | --------- | -------------------------------- |
| `dni` | string | sí        | 8 dígitos numéricos exactamente  |

**Respuesta:**

```json
{
  "dni": "12345678",
  "apaterno": "GARCIA",
  "amaterno": "LOPEZ",
  "nombre": "JUAN",
  "telf": "999999999",
  "direccion": "Av. Ejemplo 123",
  "email": "correo@mail.com",
  "ruc": "10123456789",
  "nacimiento": "1990-01-15",
  "sexo": "M"
}
```

---

### `POST /personal/eliminar_vinculo`

Eliminar un vínculo laboral. También elimina los documentos asociados (ingreso y salida) y libera la plaza poniéndola en estado `'vacante'`.

**Body:**

| Campo | Tipo   | Requerido       | Validación |
| ----- | ------ | --------------- | ---------- |
| `id`  | number | sí (vinculo id) | > 0        |

**Respuesta:**

```json
"Vínculo eliminado correctamente"
```

---

### `POST /personal/buscar_areas`

Lista de áreas disponibles.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Gerencia Municipal" }]
```

---

### `POST /personal/buscar_cargos`

Lista de cargos disponibles.

**Body:** vacío

**Respuesta:**

```json
[{ "id": 1, "nombre": "Analista" }]
```

---

### `POST /personal/upsert_evento_vinculo`

Agregar o actualizar un evento de vínculo laboral (desplazamiento, encargo, etc). Si `id` es `null`, se inserta un nuevo evento.

**Body:**

| Campo              | Tipo    | Requerido | Validación                      |
| ------------------ | ------- | --------- | ------------------------------- |
| `id`               | number? | no        |                                 |
| `vinculo_id`       | number  | sí        | > 0                             |
| `tipo_evento`      | string  | sí        | No puede estar vacío            |
| `nueva_area_id`    | number? | no        |                                 |
| `documento_inicio` | object  | sí        | Ver estructura abajo            |
| `documento_salida` | object? | no        | Ver estructura abajo            |
| `estado`           | string? | no        |                                 |

**Estructura de `documento_inicio` / `documento_salida`:**

| Campo             | Tipo    | Requerido | Validación            |
| ----------------- | ------- | --------- | --------------------- |
| `id`              | number? | no        |                       |
| `tipoDocumento`   | string? | no        |                       |
| `numeroDocumento` | number? | no        |                       |
| `añoDocumento`    | number? | no        |                       |
| `fecha`           | string  | sí        | No puede estar vacía  |
| `fechaValida`     | string? | no        |                       |
| `conv`            | number? | no        |                       |
| `descripcion`     | string  | sí        | No puede estar vacía  |
| `funcion`         | number? | no        |                       |
| `sueldo`          | number? | no        |                       |

**Respuesta:**

```json
"Operación exitosa"
```

---

### `POST /personal/delete_evento_vinculo`

Eliminar un evento de vínculo laboral y sus documentos asociados.

**Body:**

| Campo | Tipo   | Requerido | Validación |
| ----- | ------ | --------- | ---------- |
| `id`  | number | sí        | > 0        |

**Respuesta:**

```json
"Evento vinculo eliminado correctamente"
```

---

## Fileserver `/fileserver` 🔒

Todas las rutas de `/fileserver` requieren JWT, **excepto `GET /fileserver/{hash}`**.

### `GET /fileserver/{hash}`

Acceder o visualizar un archivo subido utilizando su hash. Esta ruta **no requiere JWT**, puede ser accedida directamente desde el navegador (ej. para un visor de PDFs o una etiqueta de imagen).

**Parámetros de URI:**

| Campo  | Tipo   | Requerido |
| ------ | ------ | --------- |
| `hash` | string | sí        |

**Respuesta:**

Retorna el contenido binario del archivo correspondiente. Retorna error si no se encuentra.

---

### `POST /fileserver/upload`

Subir un archivo PDF al servidor. El archivo debe pesar máximo 10 MB.

**Body (multipart/form-data):**

| Campo          | Tipo   | Requerido |
| -------------- | ------ | --------- |
| `archivo/file` | file   | sí        |
| `dni_asociado` | string | sí        |
| `documento_id` | number | no        |

**Respuesta:**

```json
[
  {
    "id": 1,
    "original_name": "mi_documento.pdf",
    "file_hash": "550e8400-e29b-41d4-a716-446655440000",
    "extension": "pdf"
  }
]
```

---

### `POST /fileserver/listar_archivos_dni`

Lista los archivos asociados a un DNI.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `dni` | string | sí        |

**Respuesta:**

```json
[
  {
    "id": 1,
    "documento_id": 10,
    "dni_asociado": "12345678",
    "original_name": "mi_documento.pdf",
    "file_hash": "550e8400-e29b-41d4-a716-446655440000",
    "extension": "pdf",
    "usuario_subida": "Admin",
    "fecha_subida": "2024-01-15 10:30:00"
  }
]
```

---

### `POST /fileserver/eliminar_archivo`

Elimina un archivo por su ID del registro y del disco.

**Body:**

| Campo | Tipo   | Requerido |
| ----- | ------ | --------- |
| `id`  | number | sí        |

**Respuesta:**

```json
"Archivo eliminado correctamente"
```
