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

#[derive(Deserialize)]
pub struct PerfilDni {
    pub dni: String,
}

pub async fn perfil_por_dni(
    data: web::Data<AppState>,
    nombre: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
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
        nombre.dni
    )
    .fetch_one(&data.db)
    .await
    .expect("REASON");

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
    let insert = sqlx::query(
        r#"
        insert into Documento (tipo, numero, year, fecha, fecha_valida, descripcion)
        values (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&doc.tipo)
    .bind(doc.numero)
    .bind(doc.año)
    .bind(&doc.fecha)
    .bind(&doc.fecha_valida)
    .bind(&doc.descripcion)
    .execute(&data.db)
    .await;

    let vinculo = sqlx::query(
        r#"
        update Vinculo set estado = 'inactivo', doc_salida_id = ? where id = ?
        "#,
    )
    .bind(insert.unwrap().last_insert_id())
    .bind(doc.id)
    .execute(&data.db)
    .await;

    match vinculo {
        Ok(result) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "registro de renuncia",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
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
    doc: web::Json<DatosBancarios>,
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
            let _ = registrar_historial(
                &req,
                &data.db,
                "editar informacion personal",
                Some(&serde_json::to_string(&perfil).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn agregar_sindicato(
    data: web::Data<AppState>,
    doc: web::Json<DocumentoSindicato>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let mut tx = data.db.begin().await.unwrap();

    let insert = sqlx::query(
        r#"
        INSERT INTO Documento (tipo,fecha, descripcion)
        VALUES ('Doc.Adm',?, ?)
        "#,
    )
    .bind(&doc.fecha)
    .bind(&doc.descripcion)
    .execute(&mut *tx)
    .await;

    let documento_id = insert.unwrap().last_insert_id();

    let _ = sqlx::query(
        r#"
        INSERT INTO vinculo_sindicato (vinculo_id, sindicato_id, documento_afiliacion)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(doc.id_vinculo)
    .bind(doc.sindicato)
    .bind(documento_id)
    .execute(&mut *tx)
    .await;

    let resultado = tx.commit().await;

    match resultado {
        Ok(_) => {
            let _ = registrar_historial(
                &req,
                &data.db,
                "afiliar al sindicato",
                Some(&serde_json::to_string(&doc).unwrap_or_default()),
            )
            .await;
            Ok(HttpResponse::Ok().json("Se registraron correctamente los datos"))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

// LEGAJOS

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
            fecha desc
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
