use crate::{
    AppState, key,
    middleware::error::{ApiError, validar},
    models::personal::{
        AsistenciaVw, ContactoEmergencia, DatosBancarios, DatosBancariosResponse,
        DeleteEventoVinculoPayload, Documento, DocumentoSindicato, EventoVinculoPayload,
        GradoAcademico, LegajoPersonal, NuevoVinculo, Perfil, PerfilInput, Persona, Vinculos,
    },
};
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self},
};

use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::Row;
use validator::Validate;

use super::registrar_historial;

#[derive(Deserialize, Validate)]
pub struct BuscarNombre {
    #[validate(length(min = 1, message = "El nombre no puede estar vacío"))]
    pub nombre: String,
}

pub async fn buscar_por_nombre(
    data: web::Data<AppState>,
    nombre: web::Json<BuscarNombre>,
) -> Result<impl Responder, ApiError> {
    validar(&nombre.0)?;
    let nombre = format!("%{}%", nombre.nombre);
    let trabajador = sqlx::query_as!(
        Persona,
        r#"
        select
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) nombre,
        v.dni,
        MIN(v.estado) AS estado,
        dg.sexo sexo
        from
        persona dg
        inner join vinculo v on dg.dni = v.dni 
        WHERE
        concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno) LIKE ?
        GROUP BY
        v.dni
        order by v.estado asc,concat_ws(" ",dg.nombre,dg.apaterno,dg.amaterno)  desc
        "#,
        nombre
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(trabajador))
}

async fn get_perfil_by_dni(data: web::Data<AppState>, dni: String) -> Result<Perfil, ApiError> {
    let key = key::key::DB_KEY;

    let trabajador = sqlx::query_as!(
        Perfil,
        r#"
        select
        p.dni,
        concat_ws(" ",p.nombre,p.apaterno,p.amaterno) nombre,
        cast(aes_decrypt(p.direccion,?) as char)  direccion,
        cast(aes_decrypt(p.telf1,?) as char)  telf,
        cast(aes_decrypt(p.email,?) as char)  email,
        p.ruc,
        p.fecha_nacimiento nacimiento,
        p.sexo,
        p.region,
        p.distrito
        from
        vinculo v
        inner join persona p on v.dni = p.dni
        where p.dni = ?
        GROUP by
        p.dni
        "#,
        key,
        key,
        key,
        dni
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener perfil".into())
    })?;

    Ok(trabajador)
}

#[derive(Deserialize, Validate)]
pub struct PerfilDni {
    #[validate(custom(function = "crate::models::personal::es_dni_valido"))]
    pub dni: String,
}

pub async fn perfil_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&nombre.0)?;
    let trabajador = get_perfil_by_dni(data, nombre.dni.clone()).await?;
    Ok(HttpResponse::Ok().json(trabajador))
}

pub async fn vinculos_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&nombre.0)?;
    let trabajador = sqlx::query_as!(
        Vinculos,
        r#"
        select * from vinculos_vigentes where dni = ? order by fecha_ingreso desc
        "#,
        nombre.dni
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener vínculos".into())
    })?;

    Ok(HttpResponse::Ok().json(trabajador))
}
pub async fn renuncia_por_vinculo(
    data: web::Data<AppState>,
    doc: web::Json<Documento>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;

    let insert_result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(&doc.descripcion)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Documento error: {}", e)))?;

    let new_doc_id = insert_result.last_insert_id();

    sqlx::query(
        r#"
        UPDATE vinculo
        SET estado = 'inactivo', doc_salida_id = ?
        WHERE id = ?
        "#,
    )
    .bind(new_doc_id)
    .bind(doc.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Vinculo error: {}", e)))?;

    sqlx::query(
        r#"
        UPDATE plaza p
        JOIN vinculo v ON p.codigo = v.plaza_id
        SET p.estado = 'vacante'
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Plaza error: {}", e)))?;

    let row = sqlx::query(
        r#"
        SELECT
          v.dni,
          cr.nombre,
          v.estado,
          ds.fecha,
          ds.descripcion,
          CONCAT_WS('-', ds.tipo_documento_id, ds.numero, ds.year) AS documento
        FROM vinculo v
        INNER JOIN documento ds ON v.doc_salida_id = ds.id
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Select after update error: {}", e)))?;

    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;

    let json_value = serde_json::json!({
        "dni": row.try_get::<Option<String>, _>("dni")
            .map_err(|e| ApiError::InternalError(format!("Row get dni error: {}", e)))?,
        "nombre": row.try_get::<Option<String>, _>("nombre")
            .map_err(|e| ApiError::InternalError(format!("Row get nombre error: {}", e)))?,
        "estado": row.try_get::<Option<String>, _>("estado")
            .map_err(|e| ApiError::InternalError(format!("Row get estado error: {}", e)))?,
        "fecha": row.try_get::<Option<NaiveDate>, _>("fecha")
            .map_err(|e| ApiError::InternalError(format!("Row get fecha error: {}", e)))?
            .map(|d| d.to_string()),
        "descripcion": row.try_get::<Option<String>, _>("descripcion")
            .map_err(|e| ApiError::InternalError(format!("Row get descripcion error: {}", e)))?,
        "documento": row.try_get::<Option<String>, _>("documento")
            .map_err(|e| ApiError::InternalError(format!("Row get documento error: {}", e)))?,
    });

    if let Err(e) = registrar_historial(
        &req,
        &data.db,
        "registro de renuncia",
        json_value.get("dni").unwrap().as_str().unwrap(),
        Some(&serde_json::to_string(&json_value).unwrap_or_default()),
    )
    .await
    {
        eprintln!("registrar_historial failed: {}", e);
    }

    Ok(HttpResponse::Ok().json(json_value))
}

pub async fn banco_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let datos = sqlx::query_as!(
        DatosBancarios,
        r#"SELECT
                cb.id,
                cb.numero_cuenta,
                upper(cb.tipo_cuenta) tipo_cuenta,
                cb.cci,
                b.nombre banco,
                cb.estado,
                "asd" as dni
                FROM
                persona p
                inner join cuentabancaria cb on cb.dni_persona = p.dni
                inner join banco b on cb.banco_id = b.id where p.dni = ? limit 1
        "#,
        dni.dni
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener datos bancarios".into())
    })?;

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn grado_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let datos = sqlx::query_as!(
        GradoAcademico,
        r#"select * from gradoacademico where dni = ?
        "#,
        dni.dni
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener grado académico".into())
    })?;

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn agregar_infobancaria(
    data: web::Data<AppState>,
    doc: web::Json<DatosBancariosResponse>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let insert = sqlx::query(
        r#"
        insert into cuentabancaria (dni_persona, numero_cuenta, tipo_cuenta, banco_id, cci,estado)
        values (?, ?, ?, ?, ?,1)
        "#,
    )
    .bind(&doc.dni)
    .bind(&doc.numero_cuenta)
    .bind(&doc.tipo_cuenta)
    .bind(doc.banco)
    .bind(&doc.cci)
    .execute(&data.db)
    .await;

    match insert {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "ingresar cuenta bancaria",
                &doc.dni,
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(format!("Database error: {}", e))),
    }
}

pub async fn editar_datos_bancarios(
    data: web::Data<AppState>,
    mut doc: web::Json<DatosBancarios>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let insert = sqlx::query(
        r#"
        update cuentabancaria set numero_cuenta = ? , tipo_cuenta = ? , banco_id = ?, cci = ? , estado = ?
        where id = ?
        "#,
    )
    .bind(&doc.numero_cuenta)
    .bind(&doc.tipo_cuenta)
    .bind(&doc.banco)
    .bind(&doc.cci)
    .bind(doc.estado)
    .bind(doc.id)
    .execute(&data.db)
    .await;

    match insert {
        Ok(result) => {
            let row = sqlx::query!("SELECT nombre FROM banco WHERE id = ?", doc.banco)
                .fetch_one(&data.db)
                .await;

            let nombre: String = row
                .map_err(|e| ApiError::InternalError(format!("Error al obtener banco: {}", e)))?
                .nombre;
            doc.banco = nombre;

            let _ = registrar_historial(
                &req,
                &data.db,
                "actualizar cuenta bancaria",
                &doc.dni,
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(format!("Database error: {}", e))),
    }
}

pub async fn upsert_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let es_actualizacion = doc.id > 0;
    let accion = if es_actualizacion {
        "editar grado academico"
    } else {
        "agrega grado academico"
    };

    let query_result = sqlx::query(
        r#"
        INSERT INTO gradoacademico (id, profesion, universidad, colegiatura, nivel_academico, abrv, dni)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE 
            profesion = VALUES(profesion),
            universidad = VALUES(universidad),
            colegiatura = VALUES(colegiatura),
            nivel_academico = VALUES(nivel_academico),
            abrv = VALUES(abrv),
            dni = VALUES(dni)
        "#,
    )
    .bind(doc.id)
    .bind(&doc.profesion)
    .bind(&doc.universidad)
    .bind(&doc.colegiatura)
    .bind(&doc.nivel_academico)
    .bind(&doc.abrv)
    .bind(&doc.dni)
    .execute(&data.db)
    .await;

    match query_result {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                accion,
                &doc.dni,
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;

            Ok(HttpResponse::Ok().json(format!(
                "Operación exitosa. Filas afectadas: {}",
                result.rows_affected()
            )))
        }
        Err(e) => Err(ApiError::InternalError(format!(
            "Error de base de datos: {}",
            e
        ))),
    }
}

pub async fn editar_perfil(
    data: web::Data<AppState>,
    perfil: web::Json<Perfil>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&perfil.0)?;
    let key = key::key::DB_KEY;
    let perfil_antes = get_perfil_by_dni(data.clone(), perfil.dni.clone()).await?;

    let insert = sqlx::query(
        r#"
        update persona set telf1 = aes_encrypt(?,?), direccion = aes_encrypt(?,?) , email = aes_encrypt(?,?), ruc = ?
        where dni = ? 
        "#,
    )
    .bind(perfil.telf.clone())
    .bind(&key)
    .bind(perfil.direccion.clone())
    .bind(&key)
    .bind(perfil.email.clone())
    .bind(&key)
    .bind(perfil.ruc.clone())
    .bind(perfil.dni.clone())
    .execute(&data.db)
    .await;

    match insert {
        Ok(result) => {
            let mut diff = serde_json::to_value(&perfil).unwrap();

            if perfil_antes.telf == perfil.telf {
                diff["telf"] = serde_json::Value::Null;
            }
            if perfil_antes.direccion == perfil.direccion {
                diff["direccion"] = serde_json::Value::Null;
            }
            if perfil_antes.email == perfil.email {
                diff["email"] = serde_json::Value::Null;
            }
            if perfil_antes.ruc == perfil.ruc {
                diff["ruc"] = serde_json::Value::Null;
            }

            let _ = registrar_historial(
                &req,
                &data.db,
                "editar informacion personal",
                &perfil.dni,
                Some(&diff.to_string()),
            )
            .await;

            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(format!("Database error: {}", e))),
    }
}

pub async fn agregar_sindicato(
    data: web::Data<AppState>,
    doc: web::Json<DocumentoSindicato>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let mut tx = data.db.begin().await?;

    let documento_id = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, fecha, descripcion)
        VALUES (4, ?, ?)
        "#,
    )
    .bind(&doc.fecha)
    .bind(&doc.descripcion)
    .execute(&mut *tx)
    .await?
    .last_insert_id();

    for vinculo in &doc.vinculos {
        sqlx::query(
            r#"
            INSERT INTO vinculo_sindicato (vinculo_id, sindicato_id, documento_afiliacion)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(vinculo.id_vinculo)
        .bind(doc.sindicato)
        .bind(documento_id)
        .execute(&mut *tx)
        .await?;

        let _ = registrar_historial(
            &req,
            &data.db,
            "afiliar al sindicato",
            &vinculo.dni,
            Some(&serde_json::to_string(&doc).unwrap_or_default()),
        )
        .await;
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
}

pub async fn personas_legajos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query(r#"select persona from legajo GROUP by persona"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError("Database consulta malformada".into())
        })?;

    let result: Vec<Value> = data
        .iter()
        .map(|row| {
            json!({
                "persona": row.get::<String, _>("persona"),

            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

pub async fn reporte_legajo(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let datos = sqlx::query_as!(
        LegajoPersonal,
        r#"select
            id,
            persona,
            dni,
            cast(fecha as char) fecha,
            estado,
            descrip,
            false as nuevo,
            user
            from
            legajo
            where
            dni = ?
            order by
            id desc
        "#,
        dni.dni
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener legajo".into())
    })?;

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn agregar_evento_legajo(
    data: web::Data<AppState>,
    doc: web::Json<LegajoPersonal>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&doc.0)?;
    let mut tx = data.db.begin().await.unwrap();

    if doc.nuevo == 1 {
        let enum_row = sqlx::query_scalar::<_, String>(
            r#"
            SELECT COLUMN_TYPE
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_NAME = 'legajo'
              AND COLUMN_NAME = 'estado'
              AND TABLE_SCHEMA = DATABASE()
            "#,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Error al obtener ENUM: {:?}", e);
            ApiError::InternalError("Error al obtener valores de ENUM".into())
        })?;

        let enum_clean = enum_row.trim_start_matches("enum(").trim_end_matches(")");
        let mut valores: Vec<String> = enum_clean
            .split(',')
            .map(|s| s.trim_matches('\'').to_string())
            .collect();

        if !valores.contains(&"persona".to_string()) {
            valores.push("persona".to_string());

            let nuevo_enum = format!(
                "ENUM({})",
                valores
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            let alter_query = format!("ALTER TABLE legajo MODIFY COLUMN estado {};", nuevo_enum);

            sqlx::query(&alter_query)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    eprintln!("Error al modificar ENUM: {:?}", e);
                    ApiError::InternalError("No se pudo modificar ENUM".into())
                })?;
        }
    }

    let _ = sqlx::query(
        r#"
        INSERT INTO legajo (persona, dni, fecha, estado, descrip,user)
        VALUES (?, ?, ?, ?, ?,?)
        "#,
    )
    .bind(&doc.persona)
    .bind(&doc.dni)
    .bind(doc.fecha.clone())
    .bind(&doc.estado)
    .bind(&doc.descrip)
    .bind(doc.user)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Error insert legajo: {:?}", e);
        ApiError::InternalError("Error al insertar legajo".into())
    })?;

    let resultado = tx.commit().await;

    match resultado {
        Ok(_) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "cambiar legajo",
                doc.dni.as_deref().unwrap_or(""),
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
        }
        Err(e) => Err(ApiError::InternalError(format!("Database error: {}", e))),
    }
}

#[derive(Deserialize, Validate)]
pub struct AsistenciaBody {
    #[validate(custom(function = "crate::models::personal::es_dni_valido"))]
    pub dni: String,
    #[validate(range(min = 1, max = 12, message = "El mes debe estar entre 1 y 12"))]
    pub mes: i32,
    #[validate(range(min = 2000, max = 2100, message = "Año fuera de rango válido"))]
    pub año: i32,
}

pub async fn report_asistencia(
    data: web::Data<AppState>,
    dni: web::Json<AsistenciaBody>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let datos = sqlx::query_as!(
        AsistenciaVw,
        r#"select * from asistenciavw where year(fecha) = ? and month(fecha) = ? and dni = ? order by fecha asc,hora asc
        "#,
        dni.año,
        dni.mes,
        dni.dni
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener asistencia".into())
    })?;

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn contacto_emergencia_add(
    data: web::Data<AppState>,
    contacto: web::Json<ContactoEmergencia>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&contacto.0)?;
    let key = key::key::DB_KEY;
    let query = sqlx::query(
        r#"
        INSERT INTO contactoemergencia (persona_dni, nombre, telefono, relacion)
        VALUES (?, ?,aes_encrypt(?,?), ?)
        ON DUPLICATE KEY UPDATE
          nombre = VALUES(nombre),
          telefono = VALUES(telefono),
          relacion = VALUES(relacion)
        "#,
    )
    .bind(&contacto.persona_dni)
    .bind(&contacto.nombre)
    .bind(&contacto.telefono)
    .bind(key)
    .bind(&contacto.relacion)
    .execute(&data.db)
    .await;

    match query {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "insert or update contacto de emergencia",
                &contacto.persona_dni,
                Some(&serde_json::to_string(&contacto.0).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(format!("Database error: {}", e))),
    }
}

pub async fn conctaco_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&dni.0)?;
    let key = key::key::DB_KEY;
    let datos = sqlx::query_as!(
        ContactoEmergencia,
        r#"select persona_dni,nombre,relacion,cast(aes_decrypt(telefono,?) as char) telefono from contactoemergencia  where persona_dni = ?
        "#,
        key,
        dni.dni
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener contacto de emergencia".into())
    })?;

    Ok(HttpResponse::Ok().json(datos))
}

#[derive(Deserialize, serde::Serialize)]
pub struct Vacante {
    pub id: Option<i32>,
    pub dni: Option<String>,
    pub nombre: Option<String>,
    pub fecha: Option<NaiveDate>,
    pub fechavalida: Option<NaiveDate>,
    pub area: Option<String>,
    pub cargo: Option<String>,
    pub codigo: Option<String>,
    pub sueldo: Option<f64>,
    pub area_id: Option<i32>,
    pub cargo_id: Option<i32>,
}

pub async fn buscar_vacantes(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let vacantes = sqlx::query_as!(
        Vacante,
        r#"
        SELECT
        v.id,
        v.dni,
        CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS nombre,
        d.fecha,
        d.fecha_valida AS fechavalida,
        ar.nombre AS area,
        ar.id as area_id,
        cr.nombre AS cargo,
        cr.id as cargo_id,
        pl.codigo,
        v.sueldo
        FROM plaza AS pl
        LEFT JOIN vinculo AS v ON pl.codigo = v.plaza_id
        LEFT JOIN documento AS d ON v.doc_salida_id = d.id
        LEFT JOIN persona AS pe ON v.dni = pe.dni
        LEFT JOIN area AS ar ON v.area_id = ar.id
        LEFT JOIN cargo AS cr ON v.cargo_id = cr.id
        WHERE 
        pl.estado = 'vacante'
        ORDER BY 
        d.fecha DESC;
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database error retrieving vacantes".into())
    })?;

    Ok(HttpResponse::Ok().json(vacantes))
}

#[derive(Deserialize)]
pub struct BuscarPlaza {
    pub codigo: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PlazaDetalle {
    pub codigo: String,
    pub cargo_estructural: Option<String>,
    pub cargo_descripcion: Option<String>,
    pub grupo_ocupacional: Option<String>,
    pub grupo_descripcion: Option<String>,
    pub condicion: Option<String>,
    pub regimen_id: i32,
    pub regimen: String,
}

pub async fn buscar_por_plaza(
    data: web::Data<AppState>,
    body: web::Json<BuscarPlaza>,
) -> Result<impl Responder, ApiError> {
    let plaza = sqlx::query_as::<_, PlazaDetalle>(
        r#"
        select
        p.codigo,
        ce.codigo cargo_estructural,
        ce.descripcion cargo_descripcion,
        go.codigo grupo_ocupacional,
        go.descripcion grupo_descripcion,
        p.condicion,
        r.id regimen_id,
        r.decreto regimen
        from
        plaza p
        inner join cargoestructural ce on p.cargoestructural = ce.codigo
        inner join gruposocupacionales go on p.grupoocupacional = go.codigo
        inner join regimen r on p.regimen = r.id
        where p.codigo = ?
        "#,
    )
    .bind(body.codigo.clone())
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database error retrieving plaza".into())
    })?;

    Ok(HttpResponse::Ok().json(plaza))
}

pub async fn registrar_trabajador(
    data: web::Data<AppState>,
    body: web::Json<NuevoVinculo>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let key = key::key::DB_KEY;
    let mut tx = data.db.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO persona (dni, nombre, apaterno, amaterno, telf1, direccion, email, ruc, fecha_nacimiento, sexo,region,distrito)
        VALUES (?, ?, ?, ?, aes_encrypt(?,?), aes_encrypt(?,?), aes_encrypt(?,?), ?, ?, ?,?,?)
        ON DUPLICATE KEY UPDATE
            nombre = VALUES(nombre),
            apaterno = VALUES(apaterno),
            amaterno = VALUES(amaterno),
            telf1 = VALUES(telf1),
            direccion = VALUES(direccion),
            email = VALUES(email),
            ruc = VALUES(ruc),
            fecha_nacimiento = VALUES(fecha_nacimiento),
            sexo = VALUES(sexo),
            region = VALUES(region),
            distrito = VALUES(distrito)
        "#,
    )
    .bind(&body.personal.dni)
    .bind(&body.personal.nombre)
    .bind(&body.personal.apaterno)
    .bind(&body.personal.amaterno)
    .bind(&body.personal.telf)
    .bind(&key)
    .bind(&body.personal.direccion)
    .bind(&key)
    .bind(&body.personal.email)
    .bind(&key)
    .bind(&body.personal.ruc)
    .bind(&body.personal.nacimiento)
    .bind(&body.personal.sexo)
    .bind(&body.personal.region)
    .bind(&body.personal.distrito)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Upsert Persona error: {}", e)))?;

    let doc_result = sqlx::query(
        r#"
        INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&body.documento.tipo)
    .bind(body.documento.numero)
    .bind(body.documento.año)
    .bind(&body.documento.fecha)
    .bind(&body.documento.fecha_valida)
    .bind(&body.documento.descripcion)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Documento error: {}", e)))?;

    let doc_id = doc_result.last_insert_id();

    sqlx::query(
        r#"
        INSERT INTO vinculo (dni, doc_ingreso_id, area_id, cargo_id, plaza_id, sueldo, estado,regimen)
        VALUES (?, ?, ?, ?, ?, ?, 'activo',?)
        "#,
    )
    .bind(&body.personal.dni)
    .bind(doc_id)
    .bind(body.area)
    .bind(body.cargo)
    .bind(&body.airshp)
    .bind(body.sueldo)
    .bind(body.regimen)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Insert Vinculo error: {}", e)))?;

    sqlx::query(
        r#"
        UPDATE plaza SET estado = 'ocupado' WHERE codigo = ?
        "#,
    )
    .bind(&body.airshp)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Update Plaza error: {}", e)))?;

    tx.commit().await?;

    let _ = registrar_historial(
        &req,
        &data.db,
        "registrar nuevo trabajador",
        &body.personal.dni,
        Some(&serde_json::to_string(&body.personal).unwrap_or_default()),
    )
    .await;

    Ok(HttpResponse::Ok().json("Trabajador registrado correctamente"))
}

pub async fn consultar_dni_reniec(
    data: web::Data<AppState>,
    body: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let key = key::key::DB_KEY;

    let local = sqlx::query_as!(
        PerfilInput,
        r#"
        SELECT
            p.dni,
            p.amaterno,
            p.apaterno,
            p.nombre,
            CAST(AES_DECRYPT(p.telf1, ?) AS CHAR) AS telf,
            CAST(AES_DECRYPT(p.direccion, ?) AS CHAR) AS direccion,
            CAST(AES_DECRYPT(p.email, ?) AS CHAR) AS email,
            p.ruc,
            p.fecha_nacimiento AS nacimiento,
            p.sexo,
            p.region,
            p.distrito
        FROM persona p
        WHERE p.dni = ?
        "#,
        key,
        key,
        key,
        body.dni
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al consultar persona local".into())
    })?;

    if let Some(perfil) = local {
        return Ok(HttpResponse::Ok().json(perfil));
    }

    let token = std::env::var("APINET")
        .map_err(|_| ApiError::InternalError("APINET no configurado".into()))?;

    let response = data
        .cliente_http
        .get("https://api.decolecta.com/v1/reniec/dni")
        .query(&[("numero", &body.dni)])
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al consultar RENIEC: {}", e)))?;

    if !response.status().is_success() {
        return Err(ApiError::InternalError(format!(
            "RENIEC respondió con error: {}",
            response.status()
        )));
    }

    let api_data: Value = response
        .json()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al parsear respuesta: {}", e)))?;

    println!("{:?}", api_data);

    let nacimiento_str = api_data["fechaNacimiento"].as_str().unwrap_or("1900-01-01");
    let nacimiento = NaiveDate::parse_from_str(nacimiento_str, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(nacimiento_str, "%d/%m/%Y"))
        .unwrap_or(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());

    let perfil = PerfilInput {
        dni: body.dni.clone(),
        apaterno: api_data["first_last_name"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        amaterno: api_data["second_last_name"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        nombre: api_data["first_name"].as_str().unwrap_or("").to_string(),
        telf: None,
        direccion: None,
        email: None,
        ruc: None,
        nacimiento,
        sexo: None,
        region: None,
        distrito: None,
    };

    Ok(HttpResponse::Ok().json(perfil))
}

#[derive(Deserialize, Validate)]
pub struct EliminarVinculoBody {
    #[validate(range(min = 1, message = "ID de vínculo inválido"))]
    pub id: i32,
}

pub async fn eliminar_vinculo(
    data: web::Data<AppState>,
    body: web::Json<EliminarVinculoBody>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let mut tx = data.db.begin().await?;

    let vinculo = sqlx::query(
        r#"
        SELECT v.dni, v.doc_ingreso_id, v.doc_salida_id, v.plaza_id
        FROM vinculo v
        WHERE v.id = ?
        "#,
    )
    .bind(body.id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error al buscar vínculo: {}", e)))?
    .ok_or_else(|| ApiError::NotFound("Vínculo no encontrado".into()))?;

    let dni: String = vinculo.get("dni");
    let doc_ingreso_id: Option<i64> = vinculo.get("doc_ingreso_id");
    let doc_salida_id: Option<i64> = vinculo.get("doc_salida_id");
    let plaza_id: Option<String> = vinculo.get("plaza_id");

    sqlx::query("DELETE FROM vinculo_sindicato WHERE vinculo_id = ?")
        .bind(body.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar sindicato: {}", e)))?;

    sqlx::query("DELETE FROM vinculo WHERE id = ?")
        .bind(body.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al eliminar vínculo: {}", e)))?;

    if let Some(doc_id) = doc_ingreso_id {
        sqlx::query("DELETE FROM documento WHERE id = ?")
            .bind(doc_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ApiError::InternalError(format!("Error al eliminar doc ingreso: {}", e))
            })?;
    }

    if let Some(doc_id) = doc_salida_id {
        sqlx::query("DELETE FROM documento WHERE id = ?")
            .bind(doc_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error al eliminar doc salida: {}", e)))?;
    }

    if let Some(ref codigo) = plaza_id {
        sqlx::query("UPDATE plaza SET estado = 'vacante' WHERE codigo = ?")
            .bind(codigo)
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error al liberar plaza: {}", e)))?;
    }

    tx.commit().await?;

    let _ = registrar_historial(
        &req,
        &data.db,
        "eliminar vínculo",
        &dni,
        Some(&format!("vinculo_id: {}, plaza: {:?}", body.id, plaza_id)),
    )
    .await;

    Ok(HttpResponse::Ok().json("Vínculo eliminado correctamente"))
}

pub async fn buscar_areas(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let areas =
        sqlx::query("SELECT id, nombre,activo,nivel FROM area where activo = true ORDER BY nombre")
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                eprintln!("Database error: {:?}", e);
                ApiError::InternalError("Error al obtener áreas".into())
            })?;

    let resultado: Vec<Value> = areas
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "nombre": row.get::<String, _>("nombre"),
                "activo": row.get::<bool, _>("activo"),
                "nivel": row.get::<Option<i32>, _>("nivel"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

pub async fn buscar_cargos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let cargos =
        sqlx::query("SELECT id, nombre,activo FROM cargo where activo = true ORDER BY nombre")
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                eprintln!("Database error: {:?}", e);
                ApiError::InternalError("Error al obtener cargos".into())
            })?;

    let resultado: Vec<Value> = cargos
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<i32, _>("id"),
                "nombre": row.get::<String, _>("nombre"),
                "activo": row.get::<bool, _>("activo"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

pub async fn upsert_evento_vinculo(
    data: web::Data<AppState>,
    payload: web::Json<EventoVinculoPayload>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&payload.0)?;
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;

    if let Some(ref doc_inicio) = payload.documento_inicio {
        let doc_id = sqlx::query(
            r#"
            INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doc_inicio.tipo)
        .bind(doc_inicio.numero)
        .bind(doc_inicio.año)
        .bind(&doc_inicio.fecha)
        .bind(&doc_inicio.fecha_valida)
        .bind(&doc_inicio.descripcion)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar documento de inicio: {}", e)))?
        .last_insert_id() as i32;

        sqlx::query(
            r#"
            INSERT INTO eventovinculo (vinculo_id, tipo_evento, nueva_area_id, documento_inicio, estado)
            VALUES (?, ?, ?, ?, 'activo')
            "#,
        )
        .bind(payload.vinculo_id)
        .bind(&payload.tipo_evento)
        .bind(payload.nueva_area_id)
        .bind(doc_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar evento: {}", e)))?;

        if payload.tipo_evento == "abandono" {
            sqlx::query("UPDATE vinculo SET estado = 'pendiente' WHERE id = ?")
                .bind(payload.vinculo_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    ApiError::InternalError(format!(
                        "Error al actualizar estado del vínculo: {}",
                        e
                    ))
                })?;
        }
    } else if let Some(ref doc_salida) = payload.documento_salida {
        let evento_id = payload.id.filter(|&id| id > 0).ok_or_else(|| {
            ApiError::BadRequest("Se requiere el id del evento para cerrarlo".into())
        })?;

        let doc_salida_id = sqlx::query(
            r#"
            INSERT INTO documento (tipo_documento_id, numero, year, fecha, fecha_valida, descripcion)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&doc_salida.tipo)
        .bind(doc_salida.numero)
        .bind(doc_salida.año)
        .bind(&doc_salida.fecha)
        .bind(&doc_salida.fecha_valida)
        .bind(&doc_salida.descripcion)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al insertar documento de salida: {}", e)))?
        .last_insert_id() as i32;

        sqlx::query(
            r#"
            UPDATE eventovinculo
            SET documento_salida = ?, estado = 'desactivado'
            WHERE id = ?
            "#,
        )
        .bind(doc_salida_id)
        .bind(evento_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error al cerrar evento: {}", e)))?;

        // Al cerrar el evento, volver el vínculo a activo
        sqlx::query("UPDATE vinculo SET estado = 'activo' WHERE id = ?")
            .bind(payload.vinculo_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ApiError::InternalError(format!("Error al reactivar el vínculo: {}", e))
            })?;
    } else {
        return Err(ApiError::BadRequest(
            "Se debe enviar documento_inicio o documento_salida".into(),
        ));
    }

    let row = sqlx::query(
        r#"
        SELECT
            pe.dni,
            CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS persona_nombre,
            cr.nombre AS cargo,
            ar.nombre AS area
        FROM vinculo v
        INNER JOIN persona pe ON v.dni = pe.dni
        LEFT JOIN cargo cr ON v.cargo_id = cr.id
        LEFT JOIN area ar ON v.area_id = ar.id
        WHERE v.id = ?
        "#,
    )
    .bind(payload.vinculo_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ApiError::InternalError(format!("Error fetching vinculo info: {}", e)))?;

    let dni: String = row.try_get("dni").unwrap_or_default();
    let persona_nombre: String = row.try_get("persona_nombre").unwrap_or_default();
    let cargo: String = row.try_get("cargo").unwrap_or_default();
    let area: String = row.try_get("area").unwrap_or_default();

    let doc_desc = if let Some(ref doc) = payload.documento_inicio {
        doc.descripcion.clone()
    } else if let Some(ref doc) = payload.documento_salida {
        doc.descripcion.clone()
    } else {
        String::new()
    };

    let accion = if payload.documento_inicio.is_some() {
        format!("registrar evento {}", payload.tipo_evento)
    } else {
        format!("cerrar evento {}", payload.tipo_evento)
    };

    let payload_historico = serde_json::json!({
        "dni": dni,
        "nombre": persona_nombre,
        "cargo": cargo,
        "area": area,
        "tipo_evento": payload.tipo_evento,
        "descripcion": doc_desc,
    });

    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;

    let _ = registrar_historial(
        &req,
        &data.db,
        &accion,
        &dni,
        Some(&payload_historico.to_string()),
    )
    .await;

    Ok(HttpResponse::Ok().json("Operación exitosa"))
}

#[derive(Deserialize, Validate)]
pub struct BuscarPorArea {
    #[validate(range(min = 1, message = "ID de área inválido"))]
    pub area_id: i32,
}

pub async fn personal_por_area(
    data: web::Data<AppState>,
    body: web::Json<BuscarPorArea>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let trabajadores = sqlx::query(
        r#"
        SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            cr.nombre AS cargo,
            ar.nombre AS area
        FROM vinculo v
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        INNER JOIN area ar ON v.area_id = ar.id
        WHERE v.area_id = ? AND v.estado = 'activo'
        ORDER BY nombre
        "#,
    )
    .bind(body.area_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener personal por área".into())
    })?;

    let resultado: Vec<serde_json::Value> = trabajadores
        .iter()
        .map(|row| {
            json!({
                "dni":    row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "cargo":  row.get::<String, _>("cargo"),
                "area":   row.get::<String, _>("area"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

// ── Buscar personal por sindicato ────────────────────────────────────────────

#[derive(Deserialize, Validate)]
pub struct BuscarPorSindicato {
    #[validate(range(min = 1, message = "ID de sindicato inválido"))]
    pub sindicato_id: i32,
}

pub async fn personal_por_sindicato(
    data: web::Data<AppState>,
    body: web::Json<BuscarPorSindicato>,
) -> Result<impl Responder, ApiError> {
    validar(&body.0)?;
    let trabajadores = sqlx::query(
        r#"
        SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            cr.nombre AS cargo,
            ar.nombre AS area,
            s.nombre  AS sindicato
        FROM vinculo v
        INNER JOIN vinculo_sindicato vs ON vs.vinculo_id = v.id
        INNER JOIN sindicato s ON vs.sindicato_id = s.id
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        INNER JOIN area ar ON v.area_id = ar.id
        WHERE vs.sindicato_id = ? AND v.estado = 'activo'
        ORDER BY nombre
        "#,
    )
    .bind(body.sindicato_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al obtener personal por sindicato".into())
    })?;

    let resultado: Vec<serde_json::Value> = trabajadores
        .iter()
        .map(|row| {
            json!({
                "dni":       row.get::<String, _>("dni"),
                "nombre":    row.get::<String, _>("nombre"),
                "cargo":     row.get::<String, _>("cargo"),
                "area":      row.get::<String, _>("area"),
                "sindicato": row.get::<String, _>("sindicato"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

// ── Delete evento vinculo ─────────────────────────────────────────────────────

pub async fn delete_evento_vinculo(
    data: web::Data<AppState>,
    payload: web::Json<DeleteEventoVinculoPayload>,
    _req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    validar(&payload.0)?;
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::InternalError(format!("DB transaction begin error: {}", e)))?;

    #[derive(sqlx::FromRow)]
    struct EventoDocs {
        documento_inicio: i32,
        documento_salida: Option<i32>,
    }

    let docs: EventoDocs =
        sqlx::query_as("SELECT documento_inicio, documento_salida FROM eventovinculo WHERE id = ?")
            .bind(payload.id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalError(format!("Error fetching eventovinculo: {}", e)))?;

    sqlx::query("DELETE FROM eventovinculo WHERE id = ?")
        .bind(payload.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error deleting eventovinculo: {}", e)))?;

    sqlx::query("DELETE FROM documento WHERE id = ?")
        .bind(docs.documento_inicio)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalError(format!("Error deleting documento inicio: {}", e)))?;

    if let Some(ds) = docs.documento_salida {
        sqlx::query("DELETE FROM documento WHERE id = ?")
            .bind(ds)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ApiError::InternalError(format!("Error deleting documento salida: {}", e))
            })?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(format!("Commit error: {}", e)))?;

    Ok(HttpResponse::Ok().json("Evento vinculo eliminado correctamente"))
}
