use sqlx::{MySqlPool, Row};
use crate::infrastructure::web::middleware::error::ApiError;
use crate::infrastructure::web::models::dash::{
    BancosReport, Cumpleaños, DataResumen, DbOrgani, ReporteRenuncias,
};
use crate::infrastructure::web::models::dash::ReporteDocumento;
use serde_json::{Value, json};
use chrono::{NaiveDate, NaiveDateTime};

pub async fn get_cumpleanos(pool: &MySqlPool) -> Result<Vec<Cumpleaños>, ApiError> {
    sqlx::query_as!(
        Cumpleaños,
        r#"
            SELECT
            p.dni,
            CONCAT_WS(' ', p.apaterno, p.amaterno, p.nombre) AS nombre,
            p.fecha_nacimiento nacimiento,
            TIMESTAMPDIFF(YEAR, p.fecha_nacimiento, CURRENT_DATE) AS edad,
            p.avatar
            FROM
            persona p
            INNER JOIN vinculo v ON p.dni = v.dni
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
            p.avatar
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

pub async fn get_personal_activo_area(pool: &MySqlPool, area_id: i32) -> Result<Vec<Value>, ApiError> {
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

pub async fn get_historial(pool: &MySqlPool, dni: &str, key: &str) -> Result<Vec<Value>, ApiError> {
    let data = sqlx::query(
        r#"
        SELECT f.operacion,cast(aes_decrypt(f.detalle,?) as char) detalle, f.fecha, u.nombre
        FROM historial f
        INNER JOIN usuario u ON f.idusuario = u.id
        WHERE f.dni = ?
        order by f.fecha desc   
        "#,
    )
    .bind(key)
    .bind(dni)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Consulta malformada".into())
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
    Ok(result)
}

pub async fn get_db_organi(pool: &MySqlPool) -> Result<Vec<DbOrgani>, ApiError> {
    sqlx::query_as::<_, DbOrgani>(
        r#"
        SELECT
            a.id,
            a.nombre AS area,
            CONCAT(p.apaterno, ' ', p.amaterno, ' ', p.nombre) AS nombre,
            CAST(p.dni AS CHAR) AS dni,
            a.nivel AS nivel
        FROM
            area a
            LEFT JOIN vinculo v ON a.id = v.area_id
            AND v.estado = 'activo'
            AND v.cargo_id IN (30, 381, 614)
            LEFT JOIN persona p ON v.dni = p.dni
        WHERE
            a.activo = 1
        GROUP BY
            a.id
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

pub async fn get_documentos(pool: &MySqlPool) -> Result<Vec<ReporteDocumento>, ApiError> {
    sqlx::query_as!(
        ReporteDocumento,
        r#"
        SELECT id, nombre, sigla FROM tipodocumento
        "#
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

pub async fn get_exportar_excel_data(pool: &MySqlPool) -> Result<Vec<sqlx::mysql::MySqlRow>, ApiError> {
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
  CONCAT(td.nombre, ' N° ', d.numero, '-', d.year, '-', td.sigla) AS documento,
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
  INNER JOIN tipodocumento td ON d.tipo_documento_id = td.id
  LEFT JOIN plaza pl ON v.plaza_id = pl.codigo
  LEFT JOIN cargoestructural ce ON pl.cargoestructural = ce.codigo
  LEFT JOIN gruposocupacionales go ON pl.grupoocupacional = go.codigo
  LEFT JOIN cuentabancaria cb ON cb.dni_persona = p.dni
  LEFT JOIN banco b ON cb.banco_id = b.id
  left join vinculo_sindicato vs on v.id = vs.vinculo_id
  left join sindicato sin on vs.sindicato_id = sin.id
WHERE
  v.estado = 'activo' OR v.estado = 'pendiente'
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
            CONCAT_WS('-', td.sigla, d.numero, d.year) AS documento,
            ar.nombre AS area,
            cr.nombre AS cargo,
            r.decreto AS regimen,
            v.sueldo,
            pl.codigo AS plaza,
            p.avatar
        FROM vinculo v
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN documento d ON v.doc_ingreso_id = d.id
        INNER JOIN tipodocumento td ON d.tipo_documento_id = td.id
        INNER JOIN area ar ON v.area_id = ar.id
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
            di.fecha AS fecha_inicio,
            di.descripcion AS descripcion_inicio,
            ds.fecha AS fecha_salida,
            ds.descripcion AS descripcion_salida,
            p.avatar
        FROM eventovinculo ev
        INNER JOIN vinculo v ON ev.vinculo_id = v.id
        INNER JOIN persona p ON v.dni = p.dni
        INNER JOIN area ar_original ON v.area_id = ar_original.id
        INNER JOIN cargo cr ON v.cargo_id = cr.id
        LEFT JOIN area ar_nueva ON ev.nueva_area_id = ar_nueva.id
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
            json!({
                "id": fila.get::<i32, _>("id"),
                "tipo_evento": fila.get::<String, _>("tipo_evento"),
                "estado": fila.try_get::<Option<String>, _>("estado").unwrap_or(None),
                "nombre": fila.get::<String, _>("nombre"),
                "dni": fila.get::<String, _>("dni"),
                "area_original": fila.get::<String, _>("area_original"),
                "area_nueva": fila.try_get::<Option<String>, _>("area_nueva").unwrap_or(None),
                "cargo": fila.get::<String, _>("cargo"),
                "fecha_inicio": fecha_inicio.map(|d| d.to_string()),
                "descripcion_inicio": fila.try_get::<Option<String>, _>("descripcion_inicio").unwrap_or(None),
                "fecha_salida": fecha_salida.map(|d| d.to_string()),
                "descripcion_salida": fila.try_get::<Option<String>, _>("descripcion_salida").unwrap_or(None),
                "avatar": fila.try_get::<Option<String>, _>("avatar").unwrap_or(None),
            })
        })
        .collect();
    Ok(resultado)
}

pub async fn get_comparar_mef_data(pool: &MySqlPool) -> Result<Vec<sqlx::mysql::MySqlRow>, ApiError> {
    sqlx::query(
        r#"
        SELECT
          p.dni, v.plaza_id, p.nombre, p.apaterno, p.amaterno,
          p.fecha_nacimiento,
          d.fecha AS fecha_ingreso,
          b.codigo AS banco,
          CASE
            WHEN cb.tipo_cuenta = 'AHORRO' THEN 'CUENTA DE AHORRO'
            WHEN cb.tipo_cuenta = 'CORRIENTE' THEN 'CUENTA CORRIENTE'
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
          COALESCE(go.codigo, '') AS codigo_grupo_ocupacional
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
