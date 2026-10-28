use crate::common::errors::ApiError;
use crate::dash::models::{
    AirhspCampoCambio, AirhspPersonaCambio, Alerta70Anos, Alertas, BancosReport,
    CatalogosValidacion, Cumpleaños, DataResumen, DatosMef, DatosSistema, DbOrgani,
    ReporteDocumento, ReporteRenuncias,
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use serde_json::{Value, json};
use sqlx::{MySqlPool, Row};

pub async fn get_cumpleanos(pool: &MySqlPool) -> Result<Vec<Cumpleaños>, ApiError> {
    sqlx::query_as!(
        Cumpleaños,
        r#"
            SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            p.fecha_nacimiento nacimiento,
            TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) AS edad,
            p.avatar,
            r.decreto AS regimen
            FROM
            persona p
            INNER JOIN vinculo v ON p.dni = v.dni
            LEFT JOIN regimen r ON v.regimen = r.id
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
            edad,
            p.avatar,
            r.decreto
            ORDER BY
            MONTH(p.fecha_nacimiento),
            DAY(p.fecha_nacimiento);
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_info_total_activos(pool: &MySqlPool) -> Result<(i64, i64), ApiError> {
    sqlx::query!(
        r#"
        select
            count(*) as "cantidad!",
            (
                select count(*) from vinculo where estado = 'activo'
            ) as "activos!"
        from vinculo
        "#
    )
    .fetch_one(pool)
    .await
    .map(|r| (r.cantidad, r.activos))
    .map_err(|e| {
        eprintln!("Error total/activos: {:?}", e);
        ApiError::InternalError("Error al obtener resumen general".into())
    })
}

pub async fn get_info_por_regimen(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(v.id) as "cantidad!",
            r.decreto as "nombre!"
        from vinculo v
        inner join regimen r on v.regimen = r.id
        where v.estado = 'activo' 
        group by r.estructura
        order by r.nombre
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por régimen".into()))
}

pub async fn get_info_por_sexo(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(v.id) as "cantidad!",
            p.sexo as "nombre!"
        from vinculo v
        inner join persona p on p.dni = v.dni
        where v.estado = 'activo'
        group by p.sexo
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por sexo".into()))
}

pub async fn get_info_por_sindicato(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        select
            count(*) as "cantidad!",
            s.nombre as "nombre!"
        from vinculo v
        inner join vinculo_sindicato vs on vs.vinculo_id = v.id
        inner join sindicato s on vs.sindicato_id = s.id
        where v.estado = 'activo'
        group by vs.sindicato_id
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por sindicato".into()))
}

pub async fn get_personal_area_report(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        select
        ar.nombre nombre,
        count(v.area_id) cantidad
        from
        vinculo v
        inner join area ar on v.area_id = ar.id
        WHERE
        v.estado = 'activo'
        and ar.activo = 1
        GROUP by
        ar.id order by
        cantidad desc
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_renuncias_ano(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        select
        ar.nombre nombre,
        count(*) cantidad
        from
        vinculo v
        inner join documento ds on v.doc_salida_id = ds.id
        inner join area ar on v.area_id = ar.id
        where
        v.estado = 'inactivo'
        and year(ds.fecha) = year(now())
        and not v.regimen = 9
        GROUP by
        ar.nombre order by
        count(*) desc
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_bancos_report(pool: &MySqlPool) -> Result<Vec<BancosReport>, ApiError> {
    sqlx::query_as!(
        BancosReport,
        r#"
        select id,nombre from banco
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_reporte_personal_activo(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
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
  vinculo v
  inner join persona p on v.dni = p.dni
  inner join cargo cr on v.cargo_id = cr.id
  inner join area ar on v.area_id = ar.id
  inner join documento dc on v.doc_ingreso_id = dc.id
  inner join regimen rg on v.regimen = rg.id
  left join documento dcs on v.doc_salida_id = dcs.id
  left join vinculo_sindicato vs on vs.vinculo_id = v.id
  left join sindicato s on vs.sindicato_id = s.id
where
  v.estado = 'activo'
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;
    let result: Vec<Value> = data
        .iter()
        .map(|row| {
            let ingreso: NaiveDate = row.get("ingreso");
            let renuncia: Option<NaiveDate> = row.try_get("renuncia").ok();
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
    Ok(result)
}

pub async fn get_personal_activo_area(
    pool: &MySqlPool,
    area_id: i32,
) -> Result<Vec<Value>, ApiError> {
    let data_rows = sqlx::query(
        r#"
select
  cast(p.dni as char) dni,
  concat(p.apaterno, ' ', p.amaterno, ' ', p.nombre) nombre,
  dc.fecha ingreso,
  dcs.fecha renuncia,
  ar.nombre area,
  cr.nombre cargo,
  s.nombre sindicato,
  rg.nombre regimen,
  p.avatar as avatar
from
  vinculo v
  inner join persona p on v.dni = p.dni
  inner join cargo cr on v.cargo_id = cr.id
  inner join area ar on v.area_id = ar.id
  inner join documento dc on v.doc_ingreso_id = dc.id
  inner join regimen rg on v.regimen = rg.id
  left join documento dcs on v.doc_salida_id = dcs.id
  left join vinculo_sindicato vs on vs.vinculo_id = v.id
  left join sindicato s on vs.sindicato_id = s.id
where
  v.estado = 'activo'
  and v.area_id = ?
        "#,
    )
    .bind(area_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;
    let result: Vec<Value> = data_rows
        .iter()
        .map(|row| {
            let ingreso: NaiveDate = row.get("ingreso");
            let renuncia: Option<NaiveDate> = row.try_get("renuncia").ok();
            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "ingreso": ingreso.to_string(),
                "renuncia": renuncia.map(|d| d.to_string()),
                "area": row.get::<String, _>("area"),
                "cargo": row.get::<String, _>("cargo"),
                "sindicato": row.try_get::<Option<String>, _>("sindicato").unwrap_or(None),
                "regimen": row.get::<String, _>("regimen"),
                "avatar": row.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();
    Ok(result)
}

pub async fn get_personal_activo_sindicato(
    pool: &MySqlPool,
    sindicato_id: Option<i32>,
    sindicato_nombre: Option<&str>,
) -> Result<Vec<Value>, ApiError> {
    let mut query = String::from(
        r#"
select
  cast(p.dni as char) dni,
  concat(p.apaterno, ' ', p.amaterno, ' ', p.nombre) nombre,
  dc.fecha ingreso,
  dcs.fecha renuncia,
  ar.nombre area,
  cr.nombre cargo,
  s.nombre sindicato,
  rg.nombre regimen,
  p.avatar as avatar
from
  vinculo v
  inner join persona p on v.dni = p.dni
  inner join cargo cr on v.cargo_id = cr.id
  inner join area ar on v.area_id = ar.id
  inner join documento dc on v.doc_ingreso_id = dc.id
  inner join regimen rg on v.regimen = rg.id
  left join documento dcs on v.doc_salida_id = dcs.id
  inner join vinculo_sindicato vs on vs.vinculo_id = v.id
  inner join sindicato s on vs.sindicato_id = s.id
where
  v.estado = 'activo'
"#,
    );

    let data_rows = if let Some(id) = sindicato_id {
        query.push_str(" and (vs.sindicato_id = ? or s.id = ?)");
        sqlx::query(&query).bind(id).bind(id).fetch_all(pool).await
    } else if let Some(nombre) = sindicato_nombre {
        query.push_str(" and (s.nombre = ? or lower(s.nombre) = lower(?))");
        sqlx::query(&query)
            .bind(nombre)
            .bind(nombre)
            .fetch_all(pool)
            .await
    } else {
        return Err(ApiError::BadRequest(
            "Se requiere parámetro sindicato o sindicato_id".into(),
        ));
    }
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    let result: Vec<Value> = data_rows
        .iter()
        .map(|row| {
            let ingreso: NaiveDate = row.get("ingreso");
            let renuncia: Option<NaiveDate> = row.try_get("renuncia").ok();
            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "ingreso": ingreso.to_string(),
                "renuncia": renuncia.map(|d| d.to_string()),
                "area": row.get::<String, _>("area"),
                "cargo": row.get::<String, _>("cargo"),
                "sindicato": row.try_get::<Option<String>, _>("sindicato").unwrap_or(None),
                "regimen": row.get::<String, _>("regimen"),
                "avatar": row.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();
    Ok(result)
}

pub async fn get_personal_activo_regimen(
    pool: &MySqlPool,
    regimen_id: Option<i32>,
    regimen_nombre: Option<&str>,
) -> Result<Vec<Value>, ApiError> {
    let mut query = String::from(
        r#"
select
  cast(p.dni as char) dni,
  concat(p.apaterno, ' ', p.amaterno, ' ', p.nombre) nombre,
  dc.fecha ingreso,
  dcs.fecha renuncia,
  ar.nombre area,
  cr.nombre cargo,
  s.nombre sindicato,
  rg.nombre regimen,
  p.avatar as avatar
from
  vinculo v
  inner join persona p on v.dni = p.dni
  inner join cargo cr on v.cargo_id = cr.id
  inner join area ar on v.area_id = ar.id
  inner join documento dc on v.doc_ingreso_id = dc.id
  inner join regimen rg on v.regimen = rg.id
  left join documento dcs on v.doc_salida_id = dcs.id
  left join vinculo_sindicato vs on vs.vinculo_id = v.id
  left join sindicato s on vs.sindicato_id = s.id
where
  v.estado = 'activo'
"#,
    );

    let data_rows = if let Some(id) = regimen_id {
        query.push_str(" and (rg.id = ? or v.regimen = ?)");
        sqlx::query(&query)
            .bind(id)
            .bind(id)
            .fetch_all(pool)
            .await
    } else if let Some(nombre) = regimen_nombre {
        query.push_str(
            " and (rg.decreto = ? or lower(rg.decreto) = lower(?) or rg.nombre = ? or lower(rg.nombre) = lower(?) or rg.estructura = ? or lower(rg.estructura) = lower(?) or rg.decreto like concat(?, '%') or rg.nombre like concat('%', ?, '%'))",
        );
        sqlx::query(&query)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .bind(nombre)
            .fetch_all(pool)
            .await
    } else {
        return Err(ApiError::BadRequest(
            "Se requiere parámetro regimen o regimen_id".into(),
        ));
    }
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    let result: Vec<Value> = data_rows
        .iter()
        .map(|row| {
            let ingreso: NaiveDate = row.get("ingreso");
            let renuncia: Option<NaiveDate> = row.try_get("renuncia").ok();
            json!({
                "dni": row.get::<String, _>("dni"),
                "nombre": row.get::<String, _>("nombre"),
                "ingreso": ingreso.to_string(),
                "renuncia": renuncia.map(|d| d.to_string()),
                "area": row.get::<String, _>("area"),
                "cargo": row.get::<String, _>("cargo"),
                "sindicato": row.try_get::<Option<String>, _>("sindicato").unwrap_or(None),
                "regimen": row.get::<String, _>("regimen"),
                "avatar": row.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();
    Ok(result)
}

pub async fn get_historial(
    pool: &MySqlPool,
    dni: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Value>, i64), ApiError> {
    let key = crate::common::audit::get_db_key();
    let limit_safe = limit.clamp(1, 200);
    let offset_safe = offset.max(0);

    let (total, data) = match dni {
        Some(dni_val) if !dni_val.trim().is_empty() => {
            let total_row =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM historial WHERE dni = ?")
                    .bind(dni_val)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| {
                        eprintln!("Database error al contar historial: {:?}", e);
                        ApiError::InternalError("Error al contar historial".into())
                    })?;

            let rows = sqlx::query(
                r#"
                SELECT f.dni, f.operacion, CAST(AES_DECRYPT(f.detalle, ?) AS CHAR) AS detalle, f.fecha, COALESCE(u.full_name, 'Sistema') AS nombre
                FROM historial f
                LEFT JOIN users u ON f.idusuario = u.id
                WHERE f.dni = ?
                ORDER BY f.fecha DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(key)
            .bind(dni_val)
            .bind(limit_safe)
            .bind(offset_safe)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error al consultar historial: {:?}", e);
                ApiError::InternalError("Error al consultar historial".into())
            })?;

            (total_row, rows)
        }
        _ => {
            let total_row = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM historial")
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    eprintln!("Database error al contar historial: {:?}", e);
                    ApiError::InternalError("Error al contar historial".into())
                })?;

            let rows = sqlx::query(
                r#"
                SELECT f.dni, f.operacion, CAST(AES_DECRYPT(f.detalle, ?) AS CHAR) AS detalle, f.fecha, COALESCE(u.full_name, 'Sistema') AS nombre
                FROM historial f
                LEFT JOIN users u ON f.idusuario = u.id
                ORDER BY f.fecha DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(key)
            .bind(limit_safe)
            .bind(offset_safe)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error al consultar historial: {:?}", e);
                ApiError::InternalError("Error al consultar historial".into())
            })?;

            (total_row, rows)
        }
    };

    let result: Vec<Value> = data
        .iter()
        .map(|row| {
            let detalle_str: Option<String> = row.get("detalle");
            let detalle_json: Value = match detalle_str {
                Some(s) => serde_json::from_str::<Value>(&s).unwrap_or_else(|_| Value::String(s)),
                None => Value::Null,
            };

            json!({
                "dni": row.get::<Option<String>, _>("dni").unwrap_or_default(),
                "operacion": row.get::<String, _>("operacion"),
                "detalle": detalle_json,
                "fecha": row.get::<Option<NaiveDateTime>, _>("fecha").map(|f| f.to_string()).unwrap_or_default(),
                "nombre": row.get::<Option<String>, _>("nombre").unwrap_or_else(|| "Sistema".to_string()),
            })
        })
        .collect();
    Ok((result, total))
}

pub async fn get_db_organi(pool: &MySqlPool) -> Result<Vec<DbOrgani>, ApiError> {
    sqlx::query_as::<_, DbOrgani>(
        r#"
        SELECT
            a.id,
            a.nombre AS area,
            CASE 
                WHEN ev.tipo_evento IS NOT NULL AND ev.tipo_evento = 'encargo_funciones' THEN
                    CONCAT(p_enc.apaterno, ' ', p_enc.amaterno, ' ', p_enc.nombre, ' (Encargado de Funciones)')
                WHEN ev.tipo_evento IS NOT NULL THEN
                    CONCAT(p_enc.apaterno, ' ', p_enc.amaterno, ' ', p_enc.nombre, ' (Encargado)')
                WHEN p_tit.dni IS NOT NULL THEN
                    CONCAT(p_tit.apaterno, ' ', p_tit.amaterno, ' ', p_tit.nombre)
                ELSE NULL
            END AS nombre,
            CAST(COALESCE(p_enc.dni, p_tit.dni) AS CHAR) AS dni,
            a.nivel AS nivel,
            CASE 
                WHEN ev.tipo_evento IS NOT NULL AND ev.tipo_evento = 'encargo_funciones' THEN 'ENCARGADO DE FUNCIONES'
                WHEN ev.tipo_evento IS NOT NULL THEN 'ENCARGADO'
                WHEN p_tit.dni IS NOT NULL THEN 'TITULAR'
                ELSE NULL
            END AS condicion
        FROM
            area a
            LEFT JOIN (
                SELECT 
                    ev_sub.nueva_area_id,
                    ev_sub.tipo_evento,
                    v_sub.dni
                FROM eventovinculo ev_sub
                INNER JOIN vinculo v_sub ON ev_sub.vinculo_id = v_sub.id
                WHERE ev_sub.tipo_evento IN ('encargo_puesto', 'encargo_funciones', 'encargatura')
                  AND (
                      (ev_sub.fecha_desde IS NOT NULL
                       AND ev_sub.fecha_desde <= CURDATE()
                       AND (ev_sub.fecha_hasta IS NULL OR ev_sub.fecha_hasta >= CURDATE()))
                      OR (ev_sub.fecha_desde IS NULL AND ev_sub.fecha_hasta IS NULL AND ev_sub.estado = 'activo')
                  )
                  AND ev_sub.id = (
                      SELECT MAX(ev2.id)
                      FROM eventovinculo ev2
                      WHERE ev2.nueva_area_id = ev_sub.nueva_area_id
                        AND ev2.tipo_evento IN ('encargo_puesto', 'encargo_funciones', 'encargatura')
                        AND (
                            (ev2.fecha_desde IS NOT NULL
                             AND ev2.fecha_desde <= CURDATE()
                             AND (ev2.fecha_hasta IS NULL OR ev2.fecha_hasta >= CURDATE()))
                            OR (ev2.fecha_desde IS NULL AND ev2.fecha_hasta IS NULL AND ev2.estado = 'activo')
                        )
                  )
            ) ev ON a.id = ev.nueva_area_id
            LEFT JOIN persona p_enc ON ev.dni = p_enc.dni
            LEFT JOIN (
                SELECT v_sub.area_id, v_sub.dni
                FROM vinculo v_sub
                WHERE v_sub.estado = 'activo'
                  AND v_sub.cargo_id IN (30, 381, 614)
                  AND v_sub.id = (
                      SELECT MAX(v2.id)
                      FROM vinculo v2
                      WHERE v2.area_id = v_sub.area_id
                        AND v2.estado = 'activo'
                        AND v2.cargo_id IN (30, 381, 614)
                  )
            ) v_tit ON a.id = v_tit.area_id
            LEFT JOIN persona p_tit ON v_tit.dni = p_tit.dni
        WHERE
            a.activo = 1
        ORDER BY
            a.id ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Consulta malformada".into())
    })
}

pub async fn get_renuncias(pool: &MySqlPool) -> Result<Vec<ReporteRenuncias>, ApiError> {
    sqlx::query_as!(
        ReporteRenuncias,
        r#"
        SELECT
            v.id,
            v.dni,
            CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS nombre,
            d.fecha,
            ar.nombre AS area,
            cr.nombre AS cargo,
            pl.codigo,
            pe.avatar
        FROM
            vinculo AS v
            INNER JOIN documento AS d ON v.doc_salida_id = d.id
            INNER JOIN persona AS pe ON v.dni = pe.dni
            INNER JOIN plaza AS pl ON v.plaza_id = pl.codigo
            INNER JOIN area AS ar ON v.area_id = ar.id
            INNER JOIN cargo AS cr ON v.cargo_id = cr.id
        WHERE
            v.estado = 'inactivo'
            AND (
                d.fecha BETWEEN DATE_SUB(CURRENT_DATE, INTERVAL 120 DAY) 
                        AND DATE_ADD(CURRENT_DATE, INTERVAL 2 DAY)
            )
        ORDER BY
            d.fecha desc;
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_tipos_documento(pool: &MySqlPool) -> Result<Vec<ReporteDocumento>, ApiError> {
    sqlx::query_as::<_, ReporteDocumento>(
        r#"
        SELECT id, nombre FROM tipo_documento ORDER BY nombre
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })
}

pub async fn get_activos_por_distrito(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    let filas = sqlx::query(
        r#"
SELECT
  p.distrito    AS distrito,
  COUNT(*)      AS cantidad
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
WHERE
  v.estado = 'activo'
  AND p.distrito IS NOT NULL
  AND p.distrito != ''
GROUP BY
  p.distrito
UNION ALL
SELECT
  'SIN ASIGNAR'  AS distrito,
  COUNT(*)       AS cantidad
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
WHERE
  v.estado = 'activo'
  AND (p.distrito IS NULL OR p.distrito = '')
ORDER BY
  distrito,
  cantidad DESC;
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;
    let resultado: Vec<Value> = filas
        .iter()
        .map(|fila| {
            json!({
                "distrito": fila.get::<String, _>("distrito"),
                "cantidad": fila.get::<i64, _>("cantidad"),
            })
        })
        .collect();
    Ok(resultado)
}

pub async fn get_exportar_excel_data(
    pool: &MySqlPool,
) -> Result<Vec<sqlx::mysql::MySqlRow>, ApiError> {
    sqlx::query(
        r#"
        SELECT
  p.dni,
  v.plaza_id,
  p.nombre,
  p.apaterno,
  p.amaterno,
  p.sexo,
  p.fecha_nacimiento,
  CONCAT(td.nombre, ' N° ', COALESCE(d.numero, ''), '-', COALESCE(d.year, ''), CASE WHEN COALESCE(da.sigla, a.sigla) IS NOT NULL THEN CONCAT('-', COALESCE(da.sigla, a.sigla)) ELSE '' END) AS documento,
  d.fecha AS ingreso,
  c.nombre AS cargo,
  a.nombre AS area,
  v.sueldo,
  r.decreto AS regimen,
  r.nombre AS regimen_nombre,
  v.estado,
  pl.condicion,
  ce.codigo AS cargo_estructural_codigo,
  ce.descripcion AS cargo_estructural,
  go.codigo AS grupo_ocupacional_codigo,
  go.descripcion AS grupo_ocupacional,
  b.nombre AS banco,
  CASE 
    WHEN cb.tipo_cuenta = 'AHORRO' THEN 'CUENTA DE AHORRO'
    WHEN cb.tipo_cuenta = 'CORRIENTE' THEN 'CUENTA CORRIENTE'
    ELSE cb.tipo_cuenta 
  END AS tipo_cuenta,
  cb.numero_cuenta,
  cb.cci,
  sin.nombre sindicato
FROM
  vinculo v
  INNER JOIN persona p ON v.dni = p.dni
  INNER JOIN cargo c ON v.cargo_id = c.id
  INNER JOIN area a ON v.area_id = a.id
  INNER JOIN regimen r ON v.regimen = r.id
  INNER JOIN documento d ON v.doc_ingreso_id = d.id
  INNER JOIN tipo_documento td ON d.tipo_documento_id = td.id 
  LEFT JOIN area da ON d.area_id = da.id
  LEFT JOIN plaza pl ON v.plaza_id = pl.codigo
  LEFT JOIN cargoestructural ce ON pl.cargoestructural = ce.codigo
  LEFT JOIN gruposocupacionales go ON pl.grupoocupacional = go.codigo
  LEFT JOIN (
    SELECT cb.dni_persona, cb.banco_id, cb.tipo_cuenta, cb.numero_cuenta, cb.cci
    FROM cuentabancaria cb
    INNER JOIN (
      SELECT dni_persona, MAX(id) AS max_id
      FROM cuentabancaria
      WHERE estado = 1
      GROUP BY dni_persona
    ) ult ON cb.id = ult.max_id
  ) cb ON cb.dni_persona = p.dni
  LEFT JOIN banco b ON cb.banco_id = b.id
  LEFT JOIN (
    SELECT vs.vinculo_id, GROUP_CONCAT(DISTINCT sin.nombre SEPARATOR ', ') AS nombre
    FROM vinculo_sindicato vs
    INNER JOIN sindicato sin ON vs.sindicato_id = sin.id
    GROUP BY vs.vinculo_id
  ) sin ON sin.vinculo_id = v.id
WHERE
  v.estado IN ('activo', 'pendiente')
ORDER BY a.nombre, p.apaterno
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al consultar datos para Excel".into())
    })
}

pub async fn get_nuevos_trabajadores(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    let filas = sqlx::query(
        r#"
        SELECT
            v.id,
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            d.fecha AS ingreso,
            CONCAT_WS('-', COALESCE(da.sigla, ar.sigla), d.numero, d.year) AS documento,
            ar.nombre AS area,
            cr.nombre AS cargo,
            r.decreto AS regimen,
            v.sueldo,
            pl.codigo AS plaza,
            p.avatar
        FROM vinculo v
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN documento d ON v.doc_ingreso_id = d.id
        INNER JOIN tipo_documento td ON d.tipo_documento_id = td.id
        INNER JOIN area ar ON v.area_id = ar.id
        LEFT JOIN area da ON d.area_id = da.id
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        INNER JOIN regimen r ON v.regimen = r.id
        LEFT JOIN plaza pl ON v.plaza_id = pl.codigo
        WHERE v.estado = 'activo'
          AND d.fecha >= DATE_SUB(CURRENT_DATE, INTERVAL 120 DAY)
        ORDER BY d.fecha DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al consultar nuevos trabajadores".into())
    })?;
    let resultado: Vec<Value> = filas
        .iter()
        .map(|fila| {
            let ingreso: Option<NaiveDate> = fila.try_get("ingreso").ok();
            json!({
                "id": fila.get::<i32, _>("id"),
                "dni": fila.get::<String, _>("dni"),
                "nombre": fila.get::<String, _>("nombre"),
                "ingreso": ingreso.map(|d| d.to_string()),
                "documento": fila.try_get::<Option<String>, _>("documento").unwrap_or(None),
                "area": fila.get::<String, _>("area"),
                "cargo": fila.get::<String, _>("cargo"),
                "regimen": fila.get::<String, _>("regimen"),
                "sueldo": fila.try_get::<Option<f64>, _>("sueldo").unwrap_or(None),
                "plaza": fila.try_get::<Option<String>, _>("plaza").unwrap_or(None),
                "avatar": fila.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();
    Ok(resultado)
}

pub async fn get_rangos_edad(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        SELECT
            CASE
                WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 18 AND 25 THEN '18-25'
                WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 26 AND 35 THEN '26-35'
                WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 36 AND 45 THEN '36-45'
                WHEN TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) BETWEEN 46 AND 55 THEN '46-55'
                ELSE '55+'
            END AS "nombre!",
            COUNT(*) AS "cantidad!"
        FROM vinculo v
        INNER JOIN persona p ON v.dni = p.dni
        WHERE v.estado = 'activo'
        GROUP BY 1
        ORDER BY MIN(TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE))
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error rangos_edad: {:?}", e);
        ApiError::InternalError("Error al obtener rangos de edad".into())
    })
}

pub async fn get_rangos_antiguedad(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    sqlx::query_as!(
        DataResumen,
        r#"
        SELECT
            CASE
                WHEN TIMESTAMPDIFF(YEAR, d.fecha, CURRENT_DATE) < 1 THEN '0-1 años'
                WHEN TIMESTAMPDIFF(YEAR, d.fecha, CURRENT_DATE) BETWEEN 1 AND 4 THEN '1-5 años'
                WHEN TIMESTAMPDIFF(YEAR, d.fecha, CURRENT_DATE) BETWEEN 5 AND 9 THEN '5-10 años'
                ELSE '+10 años'
            END AS "nombre!",
            COUNT(*) AS "cantidad!"
        FROM vinculo v
        INNER JOIN documento d ON v.doc_ingreso_id = d.id
        WHERE v.estado = 'activo'
        GROUP BY 1
        ORDER BY MIN(TIMESTAMPDIFF(YEAR, d.fecha, CURRENT_DATE))
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error rangos_antiguedad: {:?}", e);
        ApiError::InternalError("Error al obtener rangos de antigüedad".into())
    })
}

pub async fn get_reporte_eventos(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    let filas = sqlx::query(
        r#"
        SELECT
            ev.id,
            ev.tipo_evento,
            ev.estado,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            p.dni,
            ar_original.nombre AS area_original,
            ar_nueva.nombre AS area_nueva,
            cr.nombre AS cargo,
            ev.nuevo_cargo_id,
            cr_nuevo.nombre AS cargo_nuevo,
            di.fecha AS fecha_inicio,
            di.descripcion AS descripcion_inicio,
            ds.fecha AS fecha_salida,
            ds.descripcion AS descripcion_salida,
            p.avatar,
            ev.fecha_desde,
            ev.fecha_hasta
        FROM eventovinculo ev
        INNER JOIN vinculo v ON ev.vinculo_id = v.id
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN area ar_original ON v.area_id = ar_original.id
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        LEFT JOIN area ar_nueva ON ev.nueva_area_id = ar_nueva.id
        LEFT JOIN cargo cr_nuevo ON ev.nuevo_cargo_id = cr_nuevo.id
        LEFT JOIN documento di ON ev.documento_inicio = di.id
        LEFT JOIN documento ds ON ev.documento_salida = ds.id
        ORDER BY di.fecha DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al consultar eventos".into())
    })?;
    let resultado: Vec<Value> = filas
        .iter()
        .map(|fila| {
            let fecha_inicio: Option<NaiveDate> = fila.try_get("fecha_inicio").ok();
            let fecha_salida: Option<NaiveDate> = fila.try_get("fecha_salida").ok();
            let fecha_desde: Option<NaiveDate> = fila.try_get("fecha_desde").ok();
            let fecha_hasta: Option<NaiveDate> = fila.try_get("fecha_hasta").ok();
            json!({
                "id": fila.get::<i32, _>("id"),
                "tipo_evento": fila.get::<String, _>("tipo_evento"),
                "estado": fila.try_get::<Option<String>, _>("estado").unwrap_or(None),
                "nombre": fila.get::<String, _>("nombre"),
                "dni": fila.get::<String, _>("dni"),
                "area_original": fila.get::<String, _>("area_original"),
                "area_nueva": fila.try_get::<Option<String>, _>("area_nueva").unwrap_or(None),
                "cargo": fila.get::<String, _>("cargo"),
                "nuevo_cargo_id": fila.try_get::<Option<i32>, _>("nuevo_cargo_id").unwrap_or(None),
                "cargo_nuevo": fila.try_get::<Option<String>, _>("cargo_nuevo").unwrap_or(None),
                "fecha_inicio": fecha_inicio.map(|d| d.to_string()),
                "descripcion_inicio": fila.try_get::<Option<String>, _>("descripcion_inicio").unwrap_or(None),
                "fecha_salida": fecha_salida.map(|d| d.to_string()),
                "descripcion_salida": fila.try_get::<Option<String>, _>("descripcion_salida").unwrap_or(None),
                "avatar": fila.try_get::<Option<String>, _>("avatar").unwrap_or(None),
                "fecha_desde": fecha_desde.map(|d| d.to_string()),
                "fecha_hasta": fecha_hasta.map(|d| d.to_string()),
            })
        })
        .collect();
    Ok(resultado)
}

pub async fn get_comparar_mef_data(
    pool: &MySqlPool,
) -> Result<Vec<sqlx::mysql::MySqlRow>, ApiError> {
    sqlx::query(
        r#"
        SELECT
          p.dni, v.plaza_id, p.nombre, p.apaterno, p.amaterno,
          p.fecha_nacimiento,
          d.fecha AS fecha_ingreso,
          b.codigo AS banco,
          CASE
            WHEN UPPER(cb.tipo_cuenta) = 'AHORRO' THEN 'CUENTA DE AHORRO'
            WHEN UPPER(cb.tipo_cuenta) = 'CORRIENTE' THEN 'CUENTA CORRIENTE'
            ELSE cb.tipo_cuenta
          END AS tipo_cuenta,
          cb.numero_cuenta, cb.cci,
          CASE
            WHEN r.decreto = 'D.L 1057 - T' THEN 'D. LEG. 1057 CAS'
            WHEN r.decreto = 'D.L 1057-F' THEN 'D. LEG. 1057 CAS'
            WHEN r.decreto = 'D.L 1057' THEN 'D. LEG. 1057 CAS'
            ELSE upper(r.decreto)
          END AS regimen_sistema,
          COALESCE(ce.codigo, '') AS codigo_cargo_estructural,
          COALESCE(go.codigo, '') AS codigo_grupo_ocupacional,
          v.id AS vinculo_id,
          cb.id AS cuenta_id,
          v.regimen AS regimen_id
        FROM vinculo v
          INNER JOIN persona p ON v.dni = p.dni
          INNER JOIN documento d ON v.doc_ingreso_id = d.id
          LEFT JOIN cuentabancaria cb ON cb.dni_persona = p.dni
          LEFT JOIN banco b ON cb.banco_id = b.id
          LEFT JOIN regimen r ON r.id = v.regimen
          LEFT JOIN plaza pl ON v.plaza_id = pl.codigo
          LEFT JOIN cargoestructural ce ON pl.cargoestructural = ce.codigo
          LEFT JOIN gruposocupacionales go ON pl.grupoocupacional = go.codigo
        WHERE v.estado = 'activo' OR v.estado = 'pendiente'
        ORDER BY p.apaterno
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error comparar_mef: {:?}", e);
        ApiError::InternalError("Error al consultar datos propios".into())
    })
}

pub fn row_to_datos_sistema(fila: &sqlx::mysql::MySqlRow) -> DatosSistema {
    DatosSistema {
        vinculo_id: fila.try_get("vinculo_id").unwrap_or(0),
        dni: fila.get("dni"),
        plaza_id: fila.try_get("plaza_id").ok(),
        nombre: fila.get("nombre"),
        apaterno: fila.get("apaterno"),
        amaterno: fila.get("amaterno"),
        fecha_nacimiento: fila.try_get("fecha_nacimiento").ok(),
        fecha_ingreso: fila.try_get("fecha_ingreso").ok(),
        banco: fila.try_get("banco").ok(),
        tipo_cuenta: fila.try_get("tipo_cuenta").ok(),
        numero_cuenta: fila.try_get("numero_cuenta").ok(),
        cci: fila.try_get("cci").ok(),
        codigo_cargo_estructural: fila.try_get("codigo_cargo_estructural").unwrap_or_default(),
        codigo_grupo_ocupacional: fila.try_get("codigo_grupo_ocupacional").unwrap_or_default(),
        cuenta_id: fila.try_get("cuenta_id").ok(),
    }
}

pub async fn get_catalogos_airhsp(
    pool: &MySqlPool,
) -> Result<(CatalogosValidacion, std::collections::HashMap<String, i32>), ApiError> {
    use std::collections::HashMap;

    let mut catalogos = CatalogosValidacion::default();
    let mut banco_id_por_codigo: HashMap<String, i32> = HashMap::new();

    // 1. Bancos
    let bancos_rows = sqlx::query("SELECT id, UPPER(TRIM(codigo)) as codigo FROM banco")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error catalogos bancos: {:?}", e);
            ApiError::InternalError("Error al consultar catálogo de bancos".into())
        })?;
    for r in bancos_rows {
        let id: i32 = r.get("id");
        let cod: String = r.get("codigo");
        if !cod.is_empty() {
            catalogos.bancos.insert(cod.clone());
            banco_id_por_codigo.insert(cod, id);
        }
    }

    // 2. Cargo estructural
    let cargos_rows = sqlx::query("SELECT UPPER(TRIM(codigo)) as codigo FROM cargoestructural")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error catalogos cargos: {:?}", e);
            ApiError::InternalError("Error al consultar catálogo de cargos".into())
        })?;
    for r in cargos_rows {
        let cod: String = r.get("codigo");
        if !cod.is_empty() {
            catalogos.cargos_estructurales.insert(cod);
        }
    }

    // 3. Grupos ocupacionales
    let grupos_rows = sqlx::query("SELECT UPPER(TRIM(codigo)) as codigo FROM gruposocupacionales")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error catalogos grupos: {:?}", e);
            ApiError::InternalError("Error al consultar catálogo de grupos ocupacionales".into())
        })?;
    for r in grupos_rows {
        let cod: String = r.get("codigo");
        if !cod.is_empty() {
            catalogos.grupos_ocupacionales.insert(cod);
        }
    }

    // 4. Plazas existentes
    let plazas_rows = sqlx::query("SELECT UPPER(TRIM(codigo)) as codigo FROM plaza")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error catalogos plazas: {:?}", e);
            ApiError::InternalError("Error al consultar catálogo de plazas".into())
        })?;
    for r in plazas_rows {
        let cod: String = r.get("codigo");
        if !cod.is_empty() {
            catalogos.plazas_existentes.insert(cod);
        }
    }

    // 5. Plazas ocupadas por vínculos activos/pendientes
    let plazas_ocupadas_rows = sqlx::query(
        r#"
        SELECT UPPER(TRIM(v.plaza_id)) as plaza_id, v.dni
        FROM vinculo v
        WHERE v.plaza_id IS NOT NULL AND (v.estado = 'activo' OR v.estado = 'pendiente')
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error catalogos plazas ocupadas: {:?}", e);
        ApiError::InternalError("Error al consultar plazas ocupadas".into())
    })?;
    for r in plazas_ocupadas_rows {
        let pl: String = r.get("plaza_id");
        let dni: String = r.get("dni");
        if !pl.is_empty() {
            catalogos.plazas_ocupadas.insert(pl, dni);
        }
    }

    // 6. Cuentas bancarias para unicidad (todos los registros para respetar índices UNIQUE)
    let cuentas_rows = sqlx::query(
        "SELECT dni_persona, UPPER(TRIM(numero_cuenta)) as numero_cuenta, UPPER(TRIM(COALESCE(cci, ''))) as cci FROM cuentabancaria",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error catalogos cuentas: {:?}", e);
        ApiError::InternalError("Error al consultar cuentas bancarias".into())
    })?;
    for r in cuentas_rows {
        let dni: String = r.get("dni_persona");
        let num: String = r.get("numero_cuenta");
        let cci: String = r.get("cci");
        if !num.is_empty() {
            catalogos.cuentas_por_numero.insert(num, dni.clone());
        }
        if !cci.is_empty() {
            catalogos.cuentas_por_cci.insert(cci, dni);
        }
    }

    Ok((catalogos, banco_id_por_codigo))
}

pub async fn aplicar_cambios_persona_tx(
    pool: &MySqlPool,
    sistema: &DatosSistema,
    cambio: &AirhspPersonaCambio,
    mef: &DatosMef,
    banco_id_por_codigo: &std::collections::HashMap<String, i32>,
) -> Result<Vec<AirhspCampoCambio>, String> {
    let campos_validos: Vec<&AirhspCampoCambio> = cambio
        .campos
        .iter()
        .filter(|c| c.error.is_none())
        .collect();

    if campos_validos.is_empty() {
        return Ok(Vec::new());
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Error iniciando transacción: {}", e))?;

    // 1. Cambios en persona
    let nuevo_apaterno = campos_validos
        .iter()
        .find(|c| c.campo == "APELLIDO_PATERNO")
        .map(|c| c.despues.as_str())
        .unwrap_or(&sistema.apaterno);
    let nuevo_amaterno = campos_validos
        .iter()
        .find(|c| c.campo == "APELLIDO_MATERNO")
        .map(|c| c.despues.as_str())
        .unwrap_or(&sistema.amaterno);
    let nuevo_nombre = campos_validos
        .iter()
        .find(|c| c.campo == "NOMBRES")
        .map(|c| c.despues.as_str())
        .unwrap_or(&sistema.nombre);
    let nueva_fnac = match campos_validos.iter().find(|c| c.campo == "FECHA_NACIMIENTO") {
        Some(c) => NaiveDate::parse_from_str(&c.despues, "%d/%m/%Y").ok(),
        None => sistema.fecha_nacimiento,
    };

    let hay_cambio_persona = campos_validos.iter().any(|c| {
        c.campo == "APELLIDO_PATERNO"
            || c.campo == "APELLIDO_MATERNO"
            || c.campo == "NOMBRES"
            || c.campo == "FECHA_NACIMIENTO"
    });

    if hay_cambio_persona {
        let res_per = sqlx::query(
            r#"
            UPDATE persona
            SET apaterno = ?, amaterno = ?, nombre = ?, fecha_nacimiento = ?
            WHERE dni = ?
            "#,
        )
        .bind(nuevo_apaterno)
        .bind(nuevo_amaterno)
        .bind(nuevo_nombre)
        .bind(nueva_fnac)
        .bind(&sistema.dni)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Error actualizando persona: {}", e))?;

        if res_per.rows_affected() == 0 {
            return Err(format!(
                "No se encontró a la persona con DNI {} para actualizar",
                sistema.dni
            ));
        }
    }

    // 2. Plaza y vínculo
    let mut plaza_final = sistema.plaza_id.clone().unwrap_or_default();
    let cambio_plaza = campos_validos.iter().find(|c| c.campo == "CODIGO_REGISTRO");
    if let Some(cp) = cambio_plaza {
        let nueva_plaza = cp.despues.trim().to_uppercase();

        // Revalidar que la plaza no esté ocupada por otro vínculo activo/pendiente
        let ocupante_opt = sqlx::query_scalar::<_, Option<String>>(
            r#"
            SELECT dni FROM vinculo
            WHERE plaza_id = ? AND estado IN ('activo', 'pendiente') AND dni != ?
            LIMIT 1
            "#,
        )
        .bind(&nueva_plaza)
        .bind(&sistema.dni)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| format!("Error verificando si plaza {} está ocupada: {}", nueva_plaza, e))?;

        if let Some(Some(otro_dni)) = ocupante_opt {
            return Err(format!(
                "La plaza {} ya está asignada al DNI {}",
                nueva_plaza, otro_dni
            ));
        }

        if let Some(ref plaza_ant) = sistema.plaza_id {
            if !plaza_ant.is_empty() && plaza_ant != &nueva_plaza {
                let usos_count = sqlx::query_scalar::<_, i64>(
                    r#"
                    SELECT COUNT(*) FROM vinculo
                    WHERE plaza_id = ? AND estado IN ('activo', 'pendiente') AND id != ?
                    "#,
                )
                .bind(plaza_ant)
                .bind(sistema.vinculo_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| format!("Error verificando usos de plaza anterior {}: {}", plaza_ant, e))?;

                if usos_count == 0 {
                    let _ = sqlx::query("UPDATE plaza SET estado = 'vacante' WHERE codigo = ?")
                        .bind(plaza_ant)
                        .execute(&mut *tx)
                        .await;
                }
            }
        }

        let res_v = sqlx::query("UPDATE vinculo SET plaza_id = ? WHERE id = ?")
            .bind(&nueva_plaza)
            .bind(sistema.vinculo_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Error asignando nueva plaza al vínculo: {}", e))?;

        if res_v.rows_affected() == 0 {
            return Err(format!(
                "No se encontró el vínculo ID {} para asignar plaza",
                sistema.vinculo_id
            ));
        }

        plaza_final = nueva_plaza;
    }

    let mut plaza_creada = false;
    if !plaza_final.is_empty() {
        let row_plaza = sqlx::query("SELECT codigo FROM plaza WHERE codigo = ?")
            .bind(&plaza_final)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| format!("Error consultando plaza {}: {}", plaza_final, e))?;

        if row_plaza.is_none() {
            let cargo_mef = mef.codigo_cargo_estructural.trim().to_uppercase();
            let grupo_mef = mef.codigo_grupo_ocupacional.trim().to_uppercase();
            if cargo_mef.is_empty() || grupo_mef.is_empty() {
                return Err(format!(
                    "La plaza {} no existe en el sistema y AIRHSP no trae cargo/grupo válidos para crearla",
                    plaza_final
                ));
            }
            let res_p = sqlx::query(
                r#"
                INSERT INTO plaza (codigo, estado, cargoestructural, grupoocupacional, condicion, regimen)
                VALUES (?, 'ocupado', ?, ?, NULL, NULL)
                "#,
            )
            .bind(&plaza_final)
            .bind(&cargo_mef)
            .bind(&grupo_mef)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Error insertando nueva plaza {}: {}", plaza_final, e))?;

            if res_p.rows_affected() == 0 {
                return Err(format!("No se pudo crear la plaza {}", plaza_final));
            }
            plaza_creada = true;
        } else if cambio_plaza.is_some() {
            let res_p = sqlx::query("UPDATE plaza SET estado = 'ocupado' WHERE codigo = ?")
                .bind(&plaza_final)
                .execute(&mut *tx)
                .await
                .map_err(|e| format!("Error actualizando plaza a ocupado: {}", e))?;

            if res_p.rows_affected() == 0 {
                return Err(format!(
                    "No se encontró la plaza {} para actualizar estado a ocupado",
                    plaza_final
                ));
            }
        }
    }

    let cambio_cargo = campos_validos.iter().find(|c| c.campo == "CODIGO_CARGO_ESTRUCTURAL");
    let cambio_grupo = campos_validos.iter().find(|c| c.campo == "CODIGO_GRUPO_OCUPACIONAL");

    if (cambio_cargo.is_some() || cambio_grupo.is_some()) && !plaza_final.is_empty() && !plaza_creada {
        let cargo_val = cambio_cargo
            .map(|c| c.despues.as_str())
            .unwrap_or(&sistema.codigo_cargo_estructural);
        let grupo_val = cambio_grupo
            .map(|c| c.despues.as_str())
            .unwrap_or(&sistema.codigo_grupo_ocupacional);

        let res_cg = sqlx::query("UPDATE plaza SET cargoestructural = ?, grupoocupacional = ? WHERE codigo = ?")
            .bind(cargo_val)
            .bind(grupo_val)
            .bind(&plaza_final)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Error actualizando cargo/grupo de plaza {}: {}", plaza_final, e))?;

        if res_cg.rows_affected() == 0 {
            return Err(format!(
                "No se encontró la plaza {} para actualizar cargo/grupo",
                plaza_final
            ));
        }
    }

    // 3. Cuenta bancaria
    let cambio_banco = campos_validos.iter().find(|c| c.campo == "CODIGO_ENTIDAD_FINANCIERA");
    let cambio_tipo = campos_validos.iter().find(|c| c.campo == "TIPO_CUENTA_FINANCIERA");
    let cambio_num = campos_validos.iter().find(|c| c.campo == "NUMERO_CUENTA_FINANCIERA");
    let cambio_cci = campos_validos.iter().find(|c| c.campo == "CODIGO_CUENTA_INTERBANCARIA");

    let hay_cambio_banco =
        cambio_banco.is_some() || cambio_tipo.is_some() || cambio_num.is_some() || cambio_cci.is_some();

    if hay_cambio_banco {
        let cod_banco = cambio_banco
            .map(|c| c.despues.as_str())
            .or(sistema.banco.as_deref())
            .unwrap_or("");
        let b_id = match banco_id_por_codigo.get(cod_banco).copied() {
            Some(id) => id,
            None => {
                return Err(format!(
                    "No se pudo resolver el ID de la entidad financiera '{}'",
                    cod_banco
                ));
            }
        };

        let tipo_str = cambio_tipo
            .map(|c| c.despues.as_str())
            .or(sistema.tipo_cuenta.as_deref())
            .unwrap_or("");
        let t_enum = match crate::dash::service::mapear_tipo_cuenta_bd(tipo_str) {
            Some(t) => t,
            None => {
                return Err(format!(
                    "Tipo de cuenta '{}' no es compatible para guardar en base de datos",
                    tipo_str
                ));
            }
        };

        let num_cuenta = cambio_num
            .map(|c| c.despues.as_str())
            .or(sistema.numero_cuenta.as_deref())
            .unwrap_or("");
        if num_cuenta.trim().is_empty() {
            return Err("El número de cuenta bancaria no puede estar vacío".to_string());
        }

        let cci_val = cambio_cci
            .map(|c| c.despues.as_str())
            .or(sistema.cci.as_deref());
        let cci_opt = cci_val.and_then(|s| {
            let t = s.trim();
            if t.is_empty() { None } else { Some(t) }
        });

        if let Some(cta_id) = sistema.cuenta_id {
            let res_cta = sqlx::query(
                r#"
                UPDATE cuentabancaria
                SET banco_id = ?, tipo_cuenta = ?, numero_cuenta = ?, cci = ?
                WHERE id = ?
                "#,
            )
            .bind(b_id)
            .bind(t_enum)
            .bind(num_cuenta)
            .bind(cci_opt)
            .bind(cta_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Error actualizando cuenta bancaria: {}", e))?;

            if res_cta.rows_affected() == 0 {
                return Err(format!(
                    "No se encontró la cuenta bancaria ID {} para actualizar",
                    cta_id
                ));
            }
        } else {
            let res_ins = sqlx::query(
                r#"
                INSERT INTO cuentabancaria (dni_persona, numero_cuenta, tipo_cuenta, banco_id, cci, estado)
                VALUES (?, ?, ?, ?, ?, 1)
                "#,
            )
            .bind(&sistema.dni)
            .bind(num_cuenta)
            .bind(t_enum)
            .bind(b_id)
            .bind(cci_opt)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Error insertando cuenta bancaria: {}", e))?;

            if res_ins.rows_affected() == 0 {
                return Err("No se pudo insertar la cuenta bancaria".to_string());
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Error confirmando transacción: {}", e))?;

    let aplicados: Vec<AirhspCampoCambio> = campos_validos.into_iter().cloned().collect();
    Ok(aplicados)
}

pub async fn get_alerta_70_anos(pool: &MySqlPool) -> Result<Vec<Alerta70Anos>, ApiError> {
    let filas = sqlx::query(
        r#"
            SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            p.fecha_nacimiento AS nacimiento,
            TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) AS edad_actual,
            ar.nombre AS area,
            cr.nombre AS cargo,
            r.decreto AS regimen,
            pl.codigo AS plaza,
            p.avatar
        FROM
            persona p
            INNER JOIN vinculo v ON p.dni = v.dni
            INNER JOIN area ar ON v.area_id = ar.id
            INNER JOIN cargo cr ON v.cargo_id = cr.id
            LEFT JOIN regimen r ON v.regimen = r.id
            LEFT JOIN plaza pl ON v.plaza_id = pl.codigo
        WHERE
            v.estado = 'activo' AND YEAR(p.fecha_nacimiento) + 70 <= YEAR(CURRENT_DATE)
        GROUP BY
            p.dni,
            p.apaterno,
            p.amaterno,
            p.nombre,
            p.fecha_nacimiento,
            ar.nombre,
            cr.nombre,
            r.decreto,
            pl.codigo,
            p.avatar
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error alerta_70_anos: {:?}", e);
        ApiError::InternalError("Error al consultar alerta de 70 años".into())
    })?;

    let hoy = chrono::Local::now().naive_local().date();

    let resultado: Vec<Alerta70Anos> = filas
        .into_iter()
        .map(|fila| {
            let nacimiento: NaiveDate = fila.get("nacimiento");
            let edad_actual: i64 = fila.try_get("edad_actual").unwrap_or(0);
            let regimen_opt: Option<String> = fila.try_get("regimen").ok();

            let estado = match edad_actual {
                69 => {
                    let cumple_70 = NaiveDate::from_ymd_opt(
                        nacimiento.year() + 70,
                        nacimiento.month(),
                        nacimiento.day(),
                    )
                    .unwrap_or_else(|| {
                        NaiveDate::from_ymd_opt(nacimiento.year() + 70, nacimiento.month(), 28)
                            .unwrap_or(nacimiento)
                    });

                    let dias_restantes = (cumple_70 - hoy).num_days();

                    if dias_restantes > 15 {
                        Alertas::Atiempo
                    } else if dias_restantes > 5 {
                        Alertas::AlLimite
                    } else {
                        Alertas::Excedido
                    }
                }
                edad if edad >= 70 => {
                    let es_1057 = regimen_opt
                        .as_deref()
                        .map(|r| r.contains("1057"))
                        .unwrap_or(false);

                    if es_1057 {
                        Alertas::Excepcional
                    } else {
                        Alertas::Excedido
                    }
                }
                _ => Alertas::Atiempo,
            };

            Alerta70Anos {
                dni: fila.get("dni"),
                nombre: fila.try_get("nombre").ok(),
                nacimiento,
                edad_actual,
                area: fila.get("area"),
                cargo: fila.get("cargo"),
                regimen: regimen_opt,
                plaza: fila.try_get("plaza").ok(),
                avatar: fila.try_get("avatar").ok(),
                estado: Some(estado),
            }
        })
        .collect();

    Ok(resultado)
}

