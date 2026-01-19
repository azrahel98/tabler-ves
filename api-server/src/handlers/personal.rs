use crate::{
    AppState,
    middleware::error::ApiError,
    models::personal::{
        AsistenciaVw, ContactoEmergencia, DatosBancarios, DatosBancariosResponse, Documento,
        DocumentoSindicato, GradoAcademico, LegajoPersonal, Perfil, Persona, Vinculos,
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

use super::registrar_historial;

#[derive(Deserialize)]
pub struct BuscarNombre {
    pub nombre: String,
}

pub async fn buscar_por_nombre(
    data: web::Data<AppState>,
    nombre: web::Json<BuscarNombre>,
) -> Result<impl Responder, ApiError> {
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
        Persona dg
        inner join Vinculo v on dg.dni = v.dni 
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
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(trabajador))
}

async fn get_perfil_by_dni(data: web::Data<AppState>, dni: String) -> Result<Perfil, ApiError> {
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());

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
        p.sexo
        from
        Vinculo v
        inner join Persona p on v.dni = p.dni
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
    .expect("REASON");

    Ok(trabajador)
}

#[derive(Deserialize)]
pub struct PerfilDni {
    pub dni: String,
}

pub async fn perfil_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let trabajador = get_perfil_by_dni(data, nombre.dni.clone()).await.unwrap();
    Ok(HttpResponse::Ok().json(trabajador))
}

pub async fn vinculos_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let trabajador = sqlx::query_as!(
        Vinculos,
        r#"
        select * from Vinculos_vigentes where dni = ? order by fecha_ingreso desc
        "#,
        nombre.dni
    )
    .fetch_all(&data.db)
    .await
    .expect("REASON");

    Ok(HttpResponse::Ok().json(trabajador))
}
pub async fn renuncia_por_vinculo(
    data: web::Data<AppState>,
    doc: web::Json<Documento>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tx =
        data.db.begin().await.map_err(|e| {
            ApiError::InternalError(3, format!("DB transaction begin error: {}", e))
        })?;

    let insert_result = sqlx::query(
        r#"
        INSERT INTO Documento (tipo, numero, year, fecha, fecha_valida, descripcion)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(&doc.descripcion)
    .execute(&mut tx)
    .await
    .map_err(|e| ApiError::InternalError(3, format!("Insert Documento error: {}", e)))?;

    let new_doc_id = insert_result.last_insert_id();

    sqlx::query(
        r#"
        UPDATE Vinculo
        SET estado = 'inactivo', doc_salida_id = ?
        WHERE id = ?
        "#,
    )
    .bind(new_doc_id)
    .bind(doc.id)
    .execute(&mut tx)
    .await
    .map_err(|e| ApiError::InternalError(3, format!("Update Vinculo error: {}", e)))?;

    sqlx::query(
        r#"
        UPDATE Plaza p
        JOIN Vinculo v ON p.codigo = v.plaza_id
        SET p.estado = 'vacante'
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id) // Usamos el mismo ID de Vinculo
    .execute(&mut tx)
    .await
    .map_err(|e| ApiError::InternalError(3, format!("Update Plaza error: {}", e)))?;

    let row = sqlx::query(
        r#"
        SELECT
          v.dni,
          cr.nombre,
          v.estado,
          ds.fecha,
          ds.descripcion,
          CONCAT_WS('-', ds.tipo, ds.numero, ds.year) AS documento
        FROM Vinculo v
        INNER JOIN Documento ds ON v.doc_salida_id = ds.id
        INNER JOIN Cargo cr ON v.cargo_id = cr.id
        WHERE v.id = ?
        "#,
    )
    .bind(doc.id)
    .fetch_one(&mut tx)
    .await
    .map_err(|e| ApiError::InternalError(3, format!("Select after update error: {}", e)))?;

    tx.commit()
        .await
        .map_err(|e| ApiError::InternalError(3, format!("Commit error: {}", e)))?;

    let json_value = serde_json::json!({
        "dni": row.try_get::<Option<String>, _>("dni")
            .map_err(|e| ApiError::InternalError(3, format!("Row get dni error: {}", e)))?,
        "nombre": row.try_get::<Option<String>, _>("nombre")
            .map_err(|e| ApiError::InternalError(3, format!("Row get nombre error: {}", e)))?,
        "estado": row.try_get::<Option<String>, _>("estado")
            .map_err(|e| ApiError::InternalError(3, format!("Row get estado error: {}", e)))?,
        "fecha": row.try_get::<Option<NaiveDate>, _>("fecha")
            .map_err(|e| ApiError::InternalError(3, format!("Row get fecha error: {}", e)))?
            .map(|d| d.to_string()),
        "descripcion": row.try_get::<Option<String>, _>("descripcion")
            .map_err(|e| ApiError::InternalError(3, format!("Row get descripcion error: {}", e)))?,
        "documento": row.try_get::<Option<String>, _>("documento")
            .map_err(|e| ApiError::InternalError(3, format!("Row get documento error: {}", e)))?,
    });

    if let Err(e) = registrar_historial(
        &req,
        &data.db,
        "registro de renuncia",
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
                Persona p
                inner join CuentaBancaria cb on cb.dni_persona = p.dni
                inner join Banco b on cb.banco_id = b.id where p.dni = ? limit 1
        "#,
        dni.dni
    )
    .fetch_optional(&data.db)
    .await
    .expect("REASON");

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn grado_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let datos = sqlx::query_as!(
        GradoAcademico,
        r#"select id,dni,descripcion,abrv from GradoAcademico where dni = ?
        "#,
        dni.dni
    )
    .fetch_optional(&data.db)
    .await
    .expect("REASON");

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn agregar_infobancaria(
    data: web::Data<AppState>,
    doc: web::Json<DatosBancariosResponse>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let insert = sqlx::query(
        r#"
        insert into CuentaBancaria (dni_persona, numero_cuenta, tipo_cuenta, banco_id, cci,estado)
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
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn editar_datos_bancarios(
    data: web::Data<AppState>,
    mut doc: web::Json<DatosBancarios>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let insert = sqlx::query(
        r#"
        update CuentaBancaria set numero_cuenta = ? , tipo_cuenta = ? , banco_id = ?, cci = ? , estado = ?
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
            let row = sqlx::query!("SELECT nombre FROM Banco WHERE id = ?", doc.banco)
                .fetch_one(&data.db) // pool es tu conexión a la BD
                .await;

            let nombre: String = row.unwrap().nombre;
            doc.banco = nombre;

            let _ = registrar_historial(
                &req,
                &data.db,
                "actualizar cuenta bancaria",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn agregar_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let insert = sqlx::query(
        r#"
        insert into GradoAcademico (descripcion, abrv, dni)
        values (?, ?, ?)
        "#,
    )
    .bind(&doc.descripcion)
    .bind(&doc.abrv)
    .bind(&doc.dni)
    .execute(&data.db)
    .await;

    match insert {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "agrega grado academico",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn editar_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let insert = sqlx::query(
        r#"
        update GradoAcademico set descripcion = ?, abrv = ?
        where id = ? 
        "#,
    )
    .bind(&doc.descripcion)
    .bind(&doc.abrv)
    .bind(doc.id)
    .execute(&data.db)
    .await;

    match insert {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "editar grado academico",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn editar_perfil(
    data: web::Data<AppState>,
    perfil: web::Json<Perfil>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());
    let perfil_antes = get_perfil_by_dni(data.clone(), perfil.dni.clone()).await?;

    let insert = sqlx::query(
        r#"
        update Persona set telf1 = aes_encrypt(?,?), direccion = aes_encrypt(?,?) , email = aes_encrypt(?,?), ruc = ?
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
                Some(&diff.to_string()),
            )
            .await;

            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn agregar_sindicato(
    data: web::Data<AppState>,
    mut doc: web::Json<DocumentoSindicato>, // Se hace mutable para poder añadirle el DNI
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tx = data.db.begin().await?;

    let documento_id = sqlx::query(
        r#"
        INSERT INTO Documento (tipo, fecha, descripcion)
        VALUES ('Doc.Adm', ?, ?)
        "#,
    )
    .bind(&doc.fecha)
    .bind(&doc.descripcion)
    .execute(&mut *tx)
    .await?
    .last_insert_id();

    sqlx::query(
        r#"
        INSERT INTO vinculo_sindicato (vinculo_id, sindicato_id, documento_afiliacion)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(doc.id_vinculo)
    .bind(doc.sindicato)
    .bind(documento_id)
    .execute(&mut *tx)
    .await?;

    let row = sqlx::query("SELECT dni FROM vinculo WHERE id = ?")
        .bind(doc.id_vinculo)
        .fetch_one(&mut *tx) // Se usa una referencia mutable a la transacción
        .await?;

    let dni: String = row.try_get("dni")?;

    doc.0.dni = Some(dni);

    tx.commit().await?;
    let _ = registrar_historial(
        &req,
        &data.db,
        "afiliar al sindicato",
        Some(&serde_json::to_string(&doc).unwrap_or_default()),
    )
    .await;

    Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
}

// LEGAJOS

// lega
pub async fn personas_legajos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query(r#"select persona from legajo GROUP by persona"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ApiError::InternalError(3, "Database consulta malformada".into())
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
    .expect("REASON");

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn agregar_evento_legajo(
    data: web::Data<AppState>,
    doc: web::Json<LegajoPersonal>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
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
            ApiError::InternalError(1, "Error al obtener valores de ENUM".into())
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
                    ApiError::InternalError(2, "No se pudo modificar ENUM".into())
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
        ApiError::InternalError(4, "Error al insertar legajo".into())
    })?;

    let resultado = tx.commit().await;

    match resultado {
        Ok(_) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "cambiar legajo",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
        }
        Err(e) => Err(ApiError::InternalError(6, format!("Database error: {}", e))),
    }
}

#[derive(Deserialize)]
pub struct AsistenciaBody {
    pub dni: String,
    pub mes: i32,
    pub año: i32,
}

pub async fn report_asistencia(
    data: web::Data<AppState>,
    dni: web::Json<AsistenciaBody>,
) -> Result<impl Responder, ApiError> {
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
    .expect("REASON");

    println!("{:?}", datos);

    Ok(HttpResponse::Ok().json(datos))
}

pub async fn contacto_emergencia_add(
    data: web::Data<AppState>,
    contacto: web::Json<ContactoEmergencia>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());
    let query = sqlx::query(
        r#"
        INSERT INTO ContactoEmergencia (persona_dni, nombre, telefono, relacion)
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
                Some(&serde_json::to_string(&contacto.0).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn conctaco_por_dni(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());
    let datos = sqlx::query_as!(
        ContactoEmergencia,
        r#"select persona_dni,nombre,relacion,cast(aes_decrypt(telefono,?) as char) telefono from ContactoEmergencia  where persona_dni = ?
        "#,
        key,
        dni.dni
    )
    .fetch_optional(&data.db)
    .await
    .expect("REASON");

    Ok(HttpResponse::Ok().json(datos))
}

#[derive(Deserialize, serde::Serialize)]
pub struct Vacante {
    pub id: i32,
    pub dni: String,
    pub nombre: Option<String>,
    pub fecha: Option<NaiveDate>,
    pub fechavalida: Option<NaiveDate>,
    pub area: String,
    pub cargo: String,
    pub codigo: String,
    pub sueldo: Option<f64>,
}

pub async fn buscar_vacantes(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let vacantes = sqlx::query_as!(
        Vacante,
        r#"
        SELECT
        v.id,
        v.dni,
        CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) nombre,
        d.fecha,
        d.fecha_valida fechavalida,
        ar.nombre AS  area,
        cr.nombre AS  cargo,
        pl.codigo,
        v.sueldo
        FROM
        Vinculo AS v
        INNER JOIN Documento AS d ON v.doc_salida_id = d.id
        INNER JOIN Persona AS pe ON v.dni = pe.dni
        INNER JOIN Plaza AS pl ON v.plaza_id = pl.codigo
        INNER JOIN Area AS ar ON v.area_id = ar.id
        INNER JOIN Cargo AS cr ON v.cargo_id = cr.id
        WHERE
        v.estado = 'inactivo'
        AND d.fecha >= DATE_SUB(CURDATE(), INTERVAL 3 MONTH)
        ORDER BY
        d.fecha DESC
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database error retrieving vacantes".into())
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
        Plaza p
        inner join CargoEstructural ce on p.cargoestructural = ce.codigo
        inner join GruposOcupacionales go on p.grupoocupacional = go.codigo
        inner join Regimen r on p.regimen = r.id
        where p.codigo = ?
        "#,
    )
    .bind(body.codigo.clone())
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database error retrieving plaza".into())
    })?;

    Ok(HttpResponse::Ok().json(plaza))
}
