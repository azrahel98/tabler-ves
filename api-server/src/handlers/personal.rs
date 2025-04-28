use crate::{
    AppState,
    middleware::error::ApiError,
    models::personal::{
        DatosBancarios, DatosBancariosResponse, Documento, GradoAcademico, Perfil, Persona,
        Vinculos,
    },
};
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self},
};
use serde::Deserialize;

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
                cb.estado
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
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn agregar_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
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
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn editar_gradoacademico(
    data: web::Data<AppState>,
    doc: web::Json<GradoAcademico>,
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
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}

pub async fn editar_perfil(
    data: web::Data<AppState>,
    perfil: web::Json<Perfil>,
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
            Ok(HttpResponse::Ok().json(format!("Rows affected: {}", result.rows_affected())))
        }
        Err(e) => Err(ApiError::InternalError(3, format!("Database error: {}", e))),
    }
}
