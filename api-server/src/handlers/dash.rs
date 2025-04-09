use crate::{
    AppState,
    middleware::error::ApiError,
    models::dash::{Cumpleaños, CumpleañosRequest, DataResumen, ResumenResponse},
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};

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
