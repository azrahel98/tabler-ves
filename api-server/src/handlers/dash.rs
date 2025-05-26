use crate::{
    AppState,
    middleware::error::ApiError,
    models::dash::{BancosReport, Cumpleaños, CumpleañosRequest, DataResumen, ResumenResponse},
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::Row;

pub async fn cumpleaños(
    data: web::Data<AppState>,
    mes: web::Json<CumpleañosRequest>,
) -> Result<impl Responder, ApiError> {
    mes.validate()?;

    let lista = sqlx::query_as!(
        Cumpleaños,
        r#"
        select
        p.dni,
        concat_ws(" ", p.apaterno, amaterno, p.nombre) nombre,
        p.fecha_nacimiento nacimiento,
        YEAR(CURRENT_DATE) - year(p.fecha_nacimiento) edad
        from
        Persona p
        inner join Vinculo v on p.dni = v.dni
        where
        v.estado = 'activo'
        and month(p.fecha_nacimiento) = ?
        GROUP by
        p.dni
        order by
        day(p.fecha_nacimiento),
        YEAR(CURRENT_DATE) - year(p.fecha_nacimiento) asc
        "#,
        mes.mes,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(lista))
}

pub async fn info(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let (total, activos) = sqlx::query!(
        r#"
        select
            count(*) as "cantidad!",
            (
                select count(*) from Vinculo where estado = 'activo'
            ) as "activos!"
        from Vinculo
        "#
    )
    .fetch_one(&data.db)
    .await
    .map(|r| (r.cantidad, r.activos))
    .map_err(|e| {
        eprintln!("Error total/activos: {:?}", e);
        ApiError::InternalError(1, "Error al obtener resumen general".into())
    })?;

    let por_regimen = sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(v.id) as "cantidad!",
            r.decreto as "nombre!"
        from Vinculo v
        inner join Regimen r on v.regimen = r.id
        where v.estado = 'activo' 
        group by r.estructura
        order by r.nombre
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError(2, "Error al obtener resumen por régimen".into()))?;

    let por_sexo = sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(v.id) as "cantidad!",
            p.sexo as "nombre!"
        from Vinculo v
        inner join Persona p on p.dni = v.dni
        where v.estado = 'activo'
        group by p.sexo
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError(3, "Error al obtener resumen por sexo".into()))?;

    let por_sindicato = sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(*) as "cantidad!",
            s.nombre as "nombre!"
        from Vinculo v
        inner join vinculo_sindicato vs on vs.vinculo_id = v.id
        inner join Sindicato s on vs.sindicato_id = s.id
        where v.estado = 'activo'
        group by vs.sindicato_id
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError(4, "Error al obtener resumen por sindicato".into()))?;

    let response = ResumenResponse {
        total,
        activos,
        por_regimen,
        por_sexo,
        por_sindicato,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn personal_area_report(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        DataResumen,
        r#"
        select
        ar.nombre nombre,
        count(v.area_id) cantidad
        from
        Vinculo v
        inner join Area ar on v.area_id = ar.id
        WHERE
        v.estado = 'activo'
        and ar.activo = 1
        GROUP by
        ar.id order by
        cantidad desc
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

pub async fn renuncias_año(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        DataResumen,
        r#"
        select
        ar.nombre nombre,
        count(*) cantidad
        from
        Vinculo v
        inner join Documento ds on v.doc_salida_id = ds.id
        inner join Area ar on v.area_id = ar.id
        where
        v.estado = 'inactivo'
        and year(ds.fecha) = year(now())
        and not v.regimen = 9
        GROUP by
        ar.nombre order by
        count(*) desc
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

pub async fn bancos_report(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        BancosReport,
        r#"
        select id,nombre from Banco
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

pub async fn reporte_personal_activo(
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let data = sqlx::query(
        r#"
        select
        cast(p.dni as char) dni,
        concat(p.apaterno, " ", p.amaterno, " ", p.nombre) nombre,
        dc.fecha ingreso,
        dcs.fecha renuncia,
        ar.nombre area,
        cr.nombre cargo,
        s.nombre sindicato,
        rg.nombre regimen
        from
        Vinculo v
        inner join Persona p on v.dni = p.dni
        inner join Cargo cr on v.cargo_id = cr.id
        inner join Area ar on v.area_id = ar.id
        inner join Documento dc on v.doc_ingreso_id = dc.id
        inner join Regimen rg on v.regimen = rg.id
        left join Documento dcs on v.doc_salida_id = dcs.id
        left join vinculo_sindicato vs on vs.vinculo_id = v.id
        left join Sindicato s on vs.sindicato_id = s.id where v.estado = 'activo'
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Database consulta malformada".into())
    })?;

    let result: Vec<Value> = data
        .iter()
        .map(|row| {
            let ingreso: NaiveDate = row.get("ingreso");
            let renuncia: Option<NaiveDate> = row.try_get("renuncia").ok(); // puede ser NULL

            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "ingreso": ingreso.to_string(),
                "renuncia": renuncia.map(|d| d.to_string()),
                "area": row.get::<String, _>("area"),
                "cargo": row.get::<String, _>("cargo"),
                "sindicato": row.try_get::<Option<String>, _>("sindicato").unwrap_or(None),
                "regimen": row.get::<String, _>("regimen"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}
#[derive(Deserialize)]
pub struct PerfilDni {
    pub dni: String,
}
pub async fn reporte_historial(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let buscar = format!("%{}%", dni.dni);

    let data = sqlx::query(
        r#"
        SELECT f.operacion, f.detalle, f.fecha, u.nombre
        FROM historial f
        INNER JOIN usuario u ON f.idusuario = u.id
        WHERE f.detalle LIKE ?
        order by f.fecha desc   
        "#,
    )
    .bind(&buscar)
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Consulta malformada".into())
    })?;

    let result: Vec<Value> = data
        .iter()
        .map(|row| {
            json!({
                "operacion": row.get::<String, _>("operacion"),
                "detalle": row.get::<String, _>("detalle"),
                "fecha": row.get::<NaiveDateTime, _>("fecha").to_string(),
                "nombre": row.get::<String, _>("nombre"),
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}
