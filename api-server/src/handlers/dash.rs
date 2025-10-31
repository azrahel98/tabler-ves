use crate::{
    AppState,
    middleware::error::ApiError,
    models::dash::{
        BancosReport, Cumpleaños, DataResumen, DbOrgani, Organigrama, ReporteLegajo,
        ReporteRenuncias, ResumenResponse,
    },
};
use actix_web::{
    HttpResponse, Responder,
    web::{self},
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::Row;

pub async fn cumpleaños(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let lista = sqlx::query_as!(
        Cumpleaños,
        r#"
            SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            p.fecha_nacimiento nacimiento,
            TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) AS edad
            FROM
            Persona p
            INNER JOIN Vinculo v ON p.dni = v.dni
            WHERE
            v.estado = 'activo'
            AND (
                STR_TO_DATE(
                CONCAT(
                    YEAR(CURRENT_DATE),
                    '-',
                    DATE_FORMAT(p.fecha_nacimiento, '%m-%d')
                ),
                '%Y-%m-%d'
                ) BETWEEN DATE_SUB(CURRENT_DATE, INTERVAL 5 DAY)
                AND DATE_ADD(CURRENT_DATE, INTERVAL 30 DAY)
                OR STR_TO_DATE(
                CONCAT(
                    YEAR(CURRENT_DATE) + 1,
                    '-',
                    DATE_FORMAT(p.fecha_nacimiento, '%m-%d')
                ),
                '%Y-%m-%d'
                ) BETWEEN DATE_SUB(CURRENT_DATE, INTERVAL 5 DAY)
                AND DATE_ADD(CURRENT_DATE, INTERVAL 10 DAY)
            )
            GROUP BY
            p.dni,
            nombre,
            p.fecha_nacimiento,
            edad
            ORDER BY
            MONTH(p.fecha_nacimiento),
            DAY(p.fecha_nacimiento);
        "#
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
    let key = std::env::var("DB_KEY").unwrap_or("*Asdf-Xasdfadf2eee".to_string());

    let data = sqlx::query(
        r#"
        SELECT f.operacion,cast(aes_decrypt(f.detalle,?) as char) detalle, f.fecha, u.nombre
        FROM historial f
        INNER JOIN usuario u ON f.idusuario = u.id
        WHERE aes_decrypt(f.detalle,?) LIKE ?
        order by f.fecha desc   
        "#,
    )
    .bind(key.clone())
    .bind(key.clone())
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

pub async fn organigrama(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let rows = sqlx::query_as::<_, DbOrgani>(
        r#"
        SELECT
            a.id,
            a.nombre AS area,
            CONCAT(p.apaterno, ' ', p.amaterno, ' ', p.nombre) AS nombre,
            CAST(p.dni AS CHAR) AS dni,
            a.nivel AS nivel
        FROM
            Area a
            LEFT JOIN Vinculo v ON a.id = v.area_id
            AND v.estado = 'activo'
            AND v.cargo_id IN (30, 381, 614)
            LEFT JOIN Persona p ON v.dni = p.dni
        WHERE
            a.activo = 1
        GROUP BY
            a.id
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError(3, "Consulta malformada".into())
    })?;

    let mut organigrama: Vec<Organigrama> = Vec::new();

    for row in &rows {
        if row.nivel.is_none() {
            let org = Organigrama {
                id: row.id,
                area: row.area.clone(),
                dni: row.dni.clone(),
                jefe: row.nombre.clone(),
                subgerencias: Vec::new(),
            };
            organigrama.push(org);
        }
    }

    for row in &rows {
        if let Some(nivel) = row.nivel {
            for org in organigrama.iter_mut() {
                if org.id == nivel {
                    let sub = Organigrama {
                        id: row.id,
                        area: row.area.clone(),
                        dni: row.dni.clone(),
                        jefe: row.nombre.clone(),
                        subgerencias: Vec::new(),
                    };
                    org.subgerencias.push(sub);
                }
            }
        }
    }

    for row in &rows {
        if let Some(nivel) = row.nivel {
            for org in organigrama.iter_mut() {
                for sub in org.subgerencias.iter_mut() {
                    if sub.id == nivel {
                        let new_sub = Organigrama {
                            id: row.id,
                            area: row.area.clone(),
                            dni: row.dni.clone(),
                            jefe: row.nombre.clone(),
                            subgerencias: Vec::new(),
                        };
                        sub.subgerencias.push(new_sub);
                    }
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(organigrama))
}

pub async fn report_legajos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        ReporteLegajo,
        r#"
        SELECT
        l.id,
        concat_ws(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
        p.dni,
        l.estado,
        l.persona,
        u.id userid,
        u.nombre usuario,
        l.fecha
        FROM
        legajo l
        INNER JOIN (
            SELECT
            dni,
            MAX(fecha) AS ultima_fecha
            FROM
            legajo
            GROUP BY
            dni
        ) AS ultimo_registro ON l.dni = ultimo_registro.dni
        AND l.fecha = ultimo_registro.ultima_fecha
        INNER JOIN Persona p ON l.dni = p.dni
        INNER JOIN usuario u ON l.user = u.id
        WHERE
        l.estado = 'prestamo'
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

pub async fn report_renuncias(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        ReporteRenuncias,
        r#"
        SELECT
        v.id,
        v.dni,
        CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) nombre,
        d.fecha,
        d.fecha_valida fechavalida,
        ar.nombre AS  area,
        cr.nombre AS  cargo,
        pl.codigo
        FROM
        Vinculo AS v
        INNER JOIN Documento AS d ON v.doc_salida_id = d.id
        INNER JOIN Persona AS pe ON v.dni = pe.dni
        INNER JOIN Plaza AS pl ON v.plaza_id = pl.codigo
        INNER JOIN Area AS ar ON v.area_id = ar.id
        INNER JOIN Cargo AS cr ON v.cargo_id = cr.id
        WHERE
        v.estado = 'inactivo'
        AND year(d.fecha) = year(now())
        ORDER BY
        pl.codigo ASC,
        d.fecha DESC;
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
