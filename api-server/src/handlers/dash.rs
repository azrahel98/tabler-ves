use crate::{
    AppState,
    handlers::personal::PerfilDni,
    middleware::error::ApiError,
    models::dash::{
        BancosReport, Cumpleaños, DataResumen, DbOrgani, Organigrama, ReporteRenuncias,
        ResumenResponse,
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
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(lista))
}

pub async fn info(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let (total, activos) = sqlx::query!(
        r#"
        select
            count(*) as "cantidad!",
            (
                select count(*) from vinculo where estado = 'activo'
            ) as "activos!"
        from vinculo
        "#
    )
    .fetch_one(&data.db)
    .await
    .map(|r| (r.cantidad, r.activos))
    .map_err(|e| {
        eprintln!("Error total/activos: {:?}", e);
        ApiError::InternalError("Error al obtener resumen general".into())
    })?;

    let por_regimen = sqlx::query_as!(
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
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por régimen".into()))?;

    let por_sexo = sqlx::query_as!(
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
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por sexo".into()))?;

    let por_sindicato = sqlx::query_as!(
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
    .fetch_all(&data.db)
    .await
    .map_err(|_e| ApiError::InternalError("Error al obtener resumen por sindicato".into()))?;

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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

pub async fn bancos_report(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        BancosReport,
        r#"
        select id,nombre from banco
        "#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
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

pub async fn reporte_historial(
    data: web::Data<AppState>,
    dni: web::Json<PerfilDni>,
) -> Result<impl Responder, ApiError> {
    let key = std::env::var("DB_KEY").expect("DB_KEY must be set");

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
    .bind(dni.dni.clone())
    .fetch_all(&data.db)
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Consulta malformada".into())
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

pub async fn report_renuncias(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        ReporteRenuncias,
        r#"
        SELECT
            v.id,
            v.dni,
            CONCAT_WS(' ', pe.apaterno, pe.amaterno, pe.nombre) AS nombre,
            d.fecha,
            ar.nombre AS area,
            cr.nombre AS cargo,
            pl.codigo
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

#[derive(Deserialize, serde::Serialize)]
pub struct ReporteDocumento {
    pub id: i32,
    pub nombre: Option<String>,
    pub sigla: Option<String>,
}

pub async fn reporte_documentos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let data = sqlx::query_as!(
        ReporteDocumento,
        r#"
        SELECT id, nombre, sigla FROM tipodocumento
        "#
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Database consulta malformada".into())
    })?;

    Ok(HttpResponse::Ok().json(data))
}

pub async fn activos_por_distrito(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
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
    .fetch_all(&data.db)
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
    Ok(HttpResponse::Ok().json(resultado))
}

pub async fn exportar_excel(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    use rust_xlsxwriter::{Format, Workbook};

    let filas = sqlx::query(
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ApiError::InternalError("Error al consultar datos para Excel".into())
    })?;

    let mut workbook = Workbook::new();
    let hoja = workbook.add_worksheet();
    hoja.set_name("AIRHSP")
        .map_err(|e| ApiError::InternalError(format!("Error al nombrar hoja: {}", e)))?;

    let formato_cabecera = Format::new().set_bold();
    let cabeceras = [
        "UNIDAD_ORGANICA",             // 0
        "CODIGO_PUESTO_CPE",           // 1
        "ESTADO",                      // 2
        "NUMERO_DOCUMENTO_IDENTIDAD",  // 3
        "APELLIDO_PATERNO",            // 4
        "APELLIDO_MATERNO",            // 5
        "NOMBRES",                     // 6
        "CODIGO_SEXO",                 // 7
        "DESC_SEXO",                   // 8
        "FECHA_NACIMIENTO",            // 9
        "FECHA_INGRESO_PERSONAL",      // 10
        "REGIMEN_LABORAL",             // 11
        "CONDICION_LABORAL",           // 12
        "CODIGO_GRUPO_OCUPACIONAL",    // 13
        "GRUPO_OCUPACIONAL",           // 14
        "CODIGO_CARGO_ESTRUCTURAL",    // 15
        "CARGO_ESTRUCTURAL",           // 16
        "CARGO_FUNCIONAL",             // 17
        "ENTIDAD_FINANCIERA",          // 18
        "TIPO_CUENTA_FINANCIERA",      // 19
        "NUMERO_CUENTA_FINANCIERA",    // 20
        "CODIGO_CUENTA_INTERBANCARIA", // 21
        "SUELDO",                      // 22,
        "SINDICATO",
    ];

    for (col, cabecera) in cabeceras.iter().enumerate() {
        hoja.write_string_with_format(0, col as u16, *cabecera, &formato_cabecera)
            .map_err(|e| ApiError::InternalError(format!("Error escribiendo cabecera: {}", e)))?;
    }

    for (i, fila) in filas.iter().enumerate() {
        let num_fila = (i + 1) as u32;

        let dni: String = fila.get("dni");
        let apaterno: String = fila.get("apaterno");
        let amaterno: String = fila.get("amaterno");
        let nombre: String = fila.get("nombre");
        let sexo: Option<String> = fila.try_get("sexo").ok();
        let nacimiento: Option<NaiveDate> = fila.try_get("fecha_nacimiento").ok();
        let ingreso: Option<NaiveDate> = fila.try_get("ingreso").ok();
        let cargo: String = fila.get("cargo");
        let area: String = fila.get("area");
        let sueldo: Option<f64> = fila.try_get("sueldo").ok();
        let regimen: Option<String> = fila.try_get("regimen").ok();
        let plaza_id: Option<String> = fila.try_get("plaza_id").ok();
        let estado: Option<String> = fila.try_get("estado").ok();
        let condicion: Option<String> = fila.try_get("condicion").ok();
        let cargo_est_codigo: Option<String> = fila.try_get("cargo_estructural_codigo").ok();
        let cargo_est: Option<String> = fila.try_get("cargo_estructural").ok();
        let grupo_ocup_codigo: Option<String> = fila.try_get("grupo_ocupacional_codigo").ok();
        let grupo_ocup: Option<String> = fila.try_get("grupo_ocupacional").ok();
        let banco: Option<String> = fila.try_get("banco").ok();
        let tipo_cuenta: Option<String> = fila.try_get("tipo_cuenta").ok();
        let numero_cuenta: Option<String> = fila.try_get("numero_cuenta").ok();
        let cci: Option<String> = fila.try_get("cci").ok();
        let sindicato: Option<String> = fila.try_get("sindicato").ok();

        macro_rules! escribir {
            ($col:expr, $val:expr) => {
                if let Some(ref v) = $val {
                    let _ = hoja.write_string(num_fila, $col, v);
                }
            };
        }

        let _ = hoja.write_string(num_fila, 0, &area);
        escribir!(1, plaza_id);
        escribir!(2, estado);
        let _ = hoja.write_string(num_fila, 3, &dni);
        let _ = hoja.write_string(num_fila, 4, &apaterno);
        let _ = hoja.write_string(num_fila, 5, &amaterno);
        let _ = hoja.write_string(num_fila, 6, &nombre);
        if let Some(ref s) = sexo {
            let codigo_sexo = if s == "M" { "1" } else { "2" };
            let _ = hoja.write_string(num_fila, 7, codigo_sexo);
            let _ = hoja.write_string(num_fila, 8, s);
        }
        if let Some(fecha) = nacimiento {
            let _ = hoja.write_string(num_fila, 9, &fecha.format("%d/%m/%Y").to_string());
        }
        if let Some(fecha) = ingreso {
            let _ = hoja.write_string(num_fila, 10, &fecha.format("%d/%m/%Y").to_string());
        }
        escribir!(11, regimen);
        escribir!(12, condicion);
        escribir!(13, grupo_ocup_codigo);
        escribir!(14, grupo_ocup);
        escribir!(15, cargo_est_codigo);
        escribir!(16, cargo_est);
        let _ = hoja.write_string(num_fila, 17, &cargo);
        escribir!(18, banco);
        escribir!(19, tipo_cuenta);
        escribir!(20, numero_cuenta);
        escribir!(21, cci);
        if let Some(s) = sueldo {
            let _ = hoja.write_number(num_fila, 22, s);
        }
        escribir!(23, sindicato);
    }

    let buffer = workbook
        .save_to_buffer()
        .map_err(|e| ApiError::InternalError(format!("Error al generar Excel: {}", e)))?;

    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .insert_header((
            "Content-Disposition",
            "attachment; filename=\"reporte_airhsp.xlsx\"",
        ))
        .body(buffer))
}

pub async fn nuevos_trabajadores(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
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
            pl.codigo AS plaza
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
    .fetch_all(&data.db)
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
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

pub async fn reporte_eventos(data: web::Data<AppState>) -> Result<impl Responder, ApiError> {
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
            ds.descripcion AS descripcion_salida
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
    .fetch_all(&data.db)
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
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(resultado))
}

// ===========================================================================
// COMPARADOR PLANILLA PROPIA vs MEF
// ===========================================================================

struct DatosMef {
    codigo_registro: String,
    #[allow(dead_code)]
    codigo_puesto: String,
    apepat: String,
    apemat: String,
    nom: String,
    fnac: String,
    fingreso: String,
    banco: String,
    tipo_cuenta: String,
    num_cuenta: String,
    cci: String,
    regimen: String, // "CAS" | "276" | "728" | "276/728"
    codigo_grupo_ocupacional: String,
    codigo_cargo_estructural: String,
}

fn mef_celda_texto(cell: &calamine::Data) -> String {
    use calamine::Data;
    match cell {
        Data::String(s) => s.trim().to_uppercase(),
        Data::Float(f) => {
            if f.fract() == 0.0 {
                format!("{}", *f as i64)
            } else {
                format!("{}", f)
            }
        }
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string().to_uppercase(),
        _ => String::new(),
    }
}

fn mef_celda_fecha(cell: &calamine::Data) -> String {
    use calamine::{Data, DataType};
    match cell {
        Data::String(s) => {
            let s = s.trim();
            if s.is_empty() {
                return String::new();
            }
            // ISO YYYY-MM-DD o YYYY-MM-DDThh:mm:ss
            if s.len() >= 10 && s.as_bytes().get(4) == Some(&b'-') {
                return format!("{}/{}/{}", &s[8..10], &s[5..7], &s[0..4]);
            }
            s.to_uppercase()
        }
        Data::DateTime(_) | Data::Float(_) => {
            if let Some(date) = cell.as_date() {
                let date: chrono::NaiveDate = date;
                date.format("%d/%m/%Y").to_string()
            } else {
                mef_celda_texto(cell)
            }
        }
        Data::DateTimeIso(s) => {
            if s.len() >= 10 {
                format!("{}/{}/{}", &s[8..10], &s[5..7], &s[0..4])
            } else {
                s.clone()
            }
        }
        _ => String::new(),
    }
}

fn mef_get_cell(row: &[calamine::Data], col: Option<usize>, es_fecha: bool) -> String {
    match col {
        None => String::new(),
        Some(i) => {
            if i >= row.len() {
                return String::new();
            }
            if es_fecha {
                mef_celda_fecha(&row[i])
            } else {
                mef_celda_texto(&row[i])
            }
        }
    }
}

/// Parsea una hoja MEF y construye el diccionario DNI -> DatosMef.
/// `regimen_default` se aplica cuando el archivo no tiene columna de régimen
/// (p.ej. el archivo CAS es siempre "CAS").
fn mef_parsear_hoja(
    range: &calamine::Range<calamine::Data>,
    regimen_default: &str,
) -> Result<std::collections::HashMap<String, DatosMef>, ApiError> {
    use calamine::Data;
    use std::collections::HashMap;

    const MEF_HEADER_IDX: usize = 3;
    const MEF_DATA_START: usize = 4;

    let mut col_map: HashMap<String, usize> = HashMap::new();
    {
        let mut iter = range.rows();
        for _ in 0..MEF_HEADER_IDX {
            iter.next();
        }
        if let Some(header_row) = iter.next() {
            for (i, cell) in header_row.iter().enumerate() {
                if let Data::String(s) = cell {
                    col_map.insert(s.trim().to_uppercase(), i);
                }
            }
        } else {
            return Err(ApiError::BadRequest(
                "El archivo MEF no tiene fila de encabezados en la fila 4".to_string(),
            ));
        }
    }

    let col_dni = col_map
        .get("NUMERO_DOCUMENTO_IDENTIDAD")
        .copied()
        .ok_or_else(|| {
            ApiError::BadRequest("NUMERO_DOCUMENTO_IDENTIDAD no encontrado en el MEF".to_string())
        })?;

    let col_registro = col_map.get("CODIGO_REGISTRO").copied();
    let col_puesto = col_map.get("CODIGO_PUESTO_CPE").copied();
    let col_apepat = col_map.get("APELLIDO_PATERNO").copied();
    let col_apemat = col_map.get("APELLIDO_MATERNO").copied();
    let col_nom = col_map.get("NOMBRES").copied();
    let col_fnac = col_map.get("FECHA_NACIMIENTO").copied();
    let col_fingreso = col_map
        .get("FECHA_INGRESO_PERSONAL")
        .or_else(|| col_map.get("FECHA_INGRESO"))
        .copied();
    let col_banco = col_map
        .get("CODIGO_ENTIDAD_FINANCIERA")
        .or_else(|| col_map.get("ENTIDAD_FINANCIERA"))
        .copied();
    let col_tipocta = col_map.get("TIPO_CUENTA_FINANCIERA").copied();
    let col_numcta = col_map.get("NUMERO_CUENTA_FINANCIERA").copied();
    let col_cci = col_map.get("CODIGO_CUENTA_INTERBANCARIA").copied();
    let col_grupo_ocup = col_map.get("CODIGO_GRUPO_OCUPACIONAL").copied();
    let col_cargo_estr = col_map.get("CODIGO_CARGO_ESTRUCTURAL").copied();
    // Columna opcional de régimen (presente en el archivo 276/728)
    let col_regimen = col_map
        .get("REGIMEN_LABORAL")
        .or_else(|| col_map.get("CODIGO_REGIMEN_LABORAL"))
        .or_else(|| col_map.get("REGIMEN"))
        .copied();

    let mut dict: HashMap<String, DatosMef> = HashMap::new();

    for row in range.rows().skip(MEF_DATA_START) {
        if col_dni >= row.len() {
            continue;
        }
        let dni = mef_celda_texto(&row[col_dni]);
        if dni.is_empty() {
            continue;
        }
        if dict.contains_key(&dni) {
            continue; // primer duplicado gana
        }

        let regimen = if let Some(ci) = col_regimen {
            let v = mef_celda_texto(&row[ci]);
            if v.is_empty() {
                regimen_default.to_string()
            } else {
                v
            }
        } else {
            regimen_default.to_string()
        };

        dict.insert(
            dni,
            DatosMef {
                codigo_registro: mef_get_cell(row, col_registro, false),
                codigo_puesto: mef_get_cell(row, col_puesto, false),
                apepat: mef_get_cell(row, col_apepat, false),
                apemat: mef_get_cell(row, col_apemat, false),
                nom: mef_get_cell(row, col_nom, false),
                fnac: mef_get_cell(row, col_fnac, true),
                fingreso: mef_get_cell(row, col_fingreso, true),
                banco: mef_get_cell(row, col_banco, false),
                tipo_cuenta: mef_get_cell(row, col_tipocta, false),
                num_cuenta: mef_get_cell(row, col_numcta, false),
                cci: mef_get_cell(row, col_cci, false),
                codigo_grupo_ocupacional: mef_get_cell(row, col_grupo_ocup, false),
                codigo_cargo_estructural: mef_get_cell(row, col_cargo_estr, false),
                regimen,
            },
        );
    }

    Ok(dict)
}

pub async fn comparar_mef(
    data: web::Data<AppState>,
    mut payload: actix_multipart::Multipart,
) -> Result<impl Responder, ApiError> {
    use calamine::{Reader, Xlsx, open_workbook_from_rs};
    use futures_util::TryStreamExt;
    use std::collections::HashMap;
    use std::io::Cursor;

    // 1. Leer los 2 archivos MEF desde el multipart
    //    "archivo_cas"   -> régimen CAS
    //    "archivo_otros" -> régimen 276/728 (columna interna distingue 276 vs 728)
    let mut bytes_cas: Vec<u8> = Vec::new();
    let mut bytes_otros: Vec<u8> = Vec::new();

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| ApiError::InternalError(format!("Error leyendo multipart: {}", e)))?
    {
        let field_name = field
            .content_disposition()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        let buf: &mut Vec<u8> = match field_name.as_str() {
            "archivo_cas" => &mut bytes_cas,
            "archivo_otros" => &mut bytes_otros,
            // compatibilidad con llamadas de un solo archivo
            "archivo" | "file" => &mut bytes_cas,
            _ => {
                while field
                    .try_next()
                    .await
                    .map_err(|e| ApiError::InternalError(e.to_string()))?
                    .is_some()
                {}
                continue;
            }
        };

        while let Some(chunk) = field
            .try_next()
            .await
            .map_err(|e| ApiError::InternalError(format!("Error leyendo chunk: {}", e)))?
        {
            buf.extend_from_slice(&chunk);
        }
    }

    if bytes_cas.is_empty() && bytes_otros.is_empty() {
        return Err(ApiError::BadRequest(
            "No se recibió ningún archivo ('archivo_cas' y/o 'archivo_otros')".to_string(),
        ));
    }

    // 2. Parsear cada archivo y unificar en un solo diccionario
    let mut dict_mef: HashMap<String, DatosMef> = HashMap::new();

    if !bytes_cas.is_empty() {
        let cursor = Cursor::new(bytes_cas);
        let mut wb: Xlsx<_> = open_workbook_from_rs(cursor)
            .map_err(|e| ApiError::InternalError(format!("Error abriendo Excel CAS: {}", e)))?;
        let sheet = wb
            .sheet_names()
            .first()
            .ok_or_else(|| ApiError::InternalError("El archivo CAS no tiene hojas".to_string()))?
            .clone();
        let range = wb
            .worksheet_range(&sheet)
            .map_err(|e| ApiError::InternalError(format!("Error leyendo hoja CAS: {}", e)))?;
        dict_mef.extend(mef_parsear_hoja(&range, "CAS")?);
    }

    if !bytes_otros.is_empty() {
        let cursor = Cursor::new(bytes_otros);
        let mut wb: Xlsx<_> = open_workbook_from_rs(cursor)
            .map_err(|e| ApiError::InternalError(format!("Error abriendo Excel 276/728: {}", e)))?;
        let sheet = wb
            .sheet_names()
            .first()
            .ok_or_else(|| {
                ApiError::InternalError("El archivo 276/728 no tiene hojas".to_string())
            })?
            .clone();
        let range = wb
            .worksheet_range(&sheet)
            .map_err(|e| ApiError::InternalError(format!("Error leyendo hoja 276/728: {}", e)))?;
        // CAS tiene prioridad si un DNI aparece en ambos archivos
        for (dni, datos) in mef_parsear_hoja(&range, "276/728")? {
            dict_mef.entry(dni).or_insert(datos);
        }
    }

    // 3. Consultar datos propios de la BD (incluyendo régimen, grupo y cargo)
    let filas_bd = sqlx::query(
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
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        eprintln!("Database error comparar_mef: {:?}", e);
        ApiError::InternalError("Error al consultar datos propios".into())
    })?;

    // 5. Comparar registro por registro
    let etiquetas = [
        "CODIGO_REGISTRO",
        "APELLIDO_PATERNO",
        "APELLIDO_MATERNO",
        "NOMBRES",
        "FECHA_NACIMIENTO",
        "FECHA_INGRESO_PERSONAL",
        "REGIMEN_LABORAL",
        "CODIGO_GRUPO_OCUPACIONAL",
        "CODIGO_CARGO_ESTRUCTURAL",
        "CODIGO_ENTIDAD_FINANCIERA",
        "TIPO_CUENTA_FINANCIERA",
        "NUMERO_CUENTA_FINANCIERA",
        "CODIGO_CUENTA_INTERBANCARIA",
    ];

    let mut comparaciones: Vec<Value> = Vec::new();
    let mut total_ok: u64 = 0;
    let mut total_diff: u64 = 0;
    let mut total_no_encontrado: u64 = 0;
    let mut total_no_en_sistema: u64 = 0;
    let mut counter: u64 = 0;

    // Conjunto de DNIs presentes en la BD (para detectar MEF no en sistema)
    let mut dnis_sistema: std::collections::HashSet<String> = std::collections::HashSet::new();

    for fila in &filas_bd {
        let dni: String = fila.get("dni");
        let apaterno: String = fila.get("apaterno");
        let amaterno: String = fila.get("amaterno");
        let nombre: String = fila.get("nombre");
        let nacimiento: Option<NaiveDate> = fila.try_get("fecha_nacimiento").ok();
        let ingreso: Option<NaiveDate> = fila.try_get("fecha_ingreso").ok();
        let banco: Option<String> = fila.try_get("banco").ok();
        let tipo_cuenta: Option<String> = fila.try_get("tipo_cuenta").ok();
        let numero_cuenta: Option<String> = fila.try_get("numero_cuenta").ok();
        let cci: Option<String> = fila.try_get("cci").ok();
        let plaza_id: Option<String> = fila.try_get("plaza_id").ok();
        let regimen_sistema: String = fila.try_get("regimen_sistema").unwrap_or_default();
        let codigo_cargo: String = fila.try_get("codigo_cargo_estructural").unwrap_or_default();
        let codigo_grupo: String = fila.try_get("codigo_grupo_ocupacional").unwrap_or_default();

        dnis_sistema.insert(dni.clone());

        let nombre_completo = format!("{} {}, {}", apaterno.trim(), amaterno.trim(), nombre.trim());
        let fnac_str = nacimiento
            .map(|d| d.format("%d/%m/%Y").to_string())
            .unwrap_or_default();
        let fingreso_str = ingreso
            .map(|d| d.format("%d/%m/%Y").to_string())
            .unwrap_or_default();
        let banco_str = banco.unwrap_or_default().trim().to_uppercase();
        let tipo_cta_str = tipo_cuenta.unwrap_or_default().trim().to_uppercase();
        let num_cta_str = numero_cuenta.unwrap_or_default().trim().to_uppercase();
        let cci_str = cci.unwrap_or_default().trim().to_uppercase();
        let cpp_str = plaza_id.unwrap_or_default().trim().to_uppercase();

        counter += 1;

        if let Some(mef) = dict_mef.get(&dni) {
            let valores_propios = [
                cpp_str.clone(),
                apaterno.trim().to_uppercase(),
                amaterno.trim().to_uppercase(),
                nombre.trim().to_uppercase(),
                fnac_str.clone(),
                fingreso_str.clone(),
                regimen_sistema.trim().to_uppercase(),
                codigo_grupo.trim().to_uppercase(),
                codigo_cargo.trim().to_uppercase(),
                banco_str.clone(),
                tipo_cta_str.clone(),
                num_cta_str.clone(),
                cci_str.clone(),
            ];

            let valores_mef = [
                mef.codigo_registro.clone(),
                mef.apepat.clone(),
                mef.apemat.clone(),
                mef.nom.clone(),
                mef.fnac.clone(),
                mef.fingreso.clone(),
                mef.regimen.trim().to_uppercase(),
                mef.codigo_grupo_ocupacional.clone(),
                mef.codigo_cargo_estructural.clone(),
                mef.banco.clone(),
                mef.tipo_cuenta.clone(),
                mef.num_cuenta.clone(),
                mef.cci.clone(),
            ];

            for i in 0..etiquetas.len() {
                let vp = &valores_propios[i];
                let vm = &valores_mef[i];
                // Si ambos están vacíos no se cuenta como diferencia
                let igual = vp == vm || (vp.is_empty() && vm.is_empty());
                if igual {
                    total_ok += 1;
                } else {
                    total_diff += 1;
                }

                comparaciones.push(json!({
                    "num": counter,
                    "dni": dni,
                    "nombre": nombre_completo,
                    "regimen": regimen_sistema,
                    "regimen_mef": mef.regimen,
                    "codigo_registro": mef.codigo_registro,
                    "codigo_puesto_cpe": cpp_str,
                    "campo": etiquetas[i],
                    "valor_propio": vp,
                    "valor_mef": vm,
                    "resultado": if igual { "OK" } else { "DIFERENCIA" }
                }));
            }
        } else {
            total_no_encontrado += 1;
            comparaciones.push(json!({
                "num": counter,
                "dni": dni,
                "nombre": nombre_completo,
                "regimen": regimen_sistema,
                "regimen_mef": null,
                "codigo_registro": null,
                "codigo_puesto_cpe": cpp_str,
                "campo": "---",
                "valor_propio": "---",
                "valor_mef": "---",
                "resultado": "NO_EXISTE_EN_MEF"
            }));
        }
    }

    // 6. Registros del MEF que NO existen en el sistema
    let mut counter_mef: u64 = 0;
    for (dni, datos) in &dict_mef {
        if !dnis_sistema.contains(dni) {
            total_no_en_sistema += 1;
            counter_mef += 1;
            let nombre_mef = format!("{} {}, {}", datos.apepat, datos.apemat, datos.nom);
            comparaciones.push(json!({
                "num": counter + counter_mef,
                "dni": dni,
                "nombre": nombre_mef,
                "regimen": datos.regimen,
                "regimen_mef": datos.regimen,
                "codigo_registro": datos.codigo_registro,
                "codigo_puesto_cpe": datos.codigo_puesto,
                "campo": "---",
                "valor_propio": "---",
                "valor_mef": "---",
                "resultado": "NO_EXISTE_EN_SISTEMA"
            }));
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "resumen": {
            "procesados": counter,
            "encontrados_mef": dict_mef.len(),
            "ok": total_ok,
            "diferencias": total_diff,
            "no_encontrados": total_no_encontrado,
            "no_en_sistema": total_no_en_sistema,
            "fecha_comparacion": chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string()
        },
        "comparaciones": comparaciones
    })))
}

// ===========================================================================
// EXPORTAR COMPARACIÓN MEF → EXCEL
// ===========================================================================

#[derive(serde::Deserialize)]
pub struct ExportarComparacionRequest {
    pub comparaciones: Vec<Value>,
}

pub async fn exportar_comparacion_mef(
    body: web::Json<ExportarComparacionRequest>,
) -> Result<impl Responder, ApiError> {
    use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook};

    let comparaciones = &body.comparaciones;

    let mut workbook = Workbook::new();

    let fmt_cab = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x1F3864))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);

    let fmt_diferencia = Format::new()
        .set_background_color(Color::RGB(0xFFF3CD))
        .set_font_color(Color::RGB(0x856404));
    let fmt_no_mef = Format::new()
        .set_background_color(Color::RGB(0xF8D7DA))
        .set_font_color(Color::RGB(0x721C24));
    let fmt_no_sistema = Format::new()
        .set_background_color(Color::RGB(0xE2D9F3))
        .set_font_color(Color::RGB(0x4A235A));
    let fmt_ok = Format::new()
        .set_background_color(Color::RGB(0xD4EDDA))
        .set_font_color(Color::RGB(0x155724));

    let cabeceras = [
        "#", "DNI", "NOMBRE", "RÉGIMEN", "CAMPO", "SISTEMA", "MEF", "ESTADO",
    ];

    // Hojas: una por categoría + una con todo
    let categorias: &[(&str, Option<&str>)] = &[
        ("DIFERENCIAS", Some("DIFERENCIA")),
        ("SOLO EN SISTEMA", Some("NO_EXISTE_EN_MEF")),
        ("SOLO EN MEF", Some("NO_EXISTE_EN_SISTEMA")),
        ("CORRECTOS", Some("OK")),
        ("TODOS", None),
    ];

    for (nombre_hoja, filtro) in categorias {
        let hoja = workbook.add_worksheet();
        hoja.set_name(*nombre_hoja)
            .map_err(|e| ApiError::InternalError(format!("Error al nombrar hoja: {}", e)))?;

        for (col, cab) in cabeceras.iter().enumerate() {
            hoja.write_string_with_format(0, col as u16, *cab, &fmt_cab)
                .map_err(|e| ApiError::InternalError(format!("Error cabecera: {}", e)))?;
        }
        hoja.set_row_height(0, 18.0)
            .map_err(|e| ApiError::InternalError(format!("Error alto fila: {}", e)))?;
        let _ = hoja.set_column_width(2, 35); // Nombre
        let _ = hoja.set_column_width(4, 30); // Campo
        let _ = hoja.set_column_width(5, 25); // Sistema
        let _ = hoja.set_column_width(6, 25); // MEF
        let _ = hoja.set_column_width(7, 22); // Estado

        let mut fila_excel = 1u32;

        for comp in comparaciones.iter() {
            let resultado = comp["resultado"].as_str().unwrap_or("");

            if let Some(f) = filtro {
                if resultado != *f {
                    continue;
                }
            }

            let fmt_fila = match resultado {
                "DIFERENCIA" => &fmt_diferencia,
                "NO_EXISTE_EN_MEF" => &fmt_no_mef,
                "NO_EXISTE_EN_SISTEMA" => &fmt_no_sistema,
                "OK" => &fmt_ok,
                _ => &fmt_ok,
            };

            let etiqueta_estado = match resultado {
                "OK" => "OK",
                "DIFERENCIA" => "Diferencia",
                "NO_EXISTE_EN_MEF" => "Solo en sistema",
                "NO_EXISTE_EN_SISTEMA" => "Solo en MEF",
                _ => resultado,
            };

            let num = comp["num"].as_f64().unwrap_or(0.0);
            let _ = hoja.write_number_with_format(fila_excel, 0, num, fmt_fila);
            let _ = hoja.write_string_with_format(
                fila_excel,
                1,
                comp["dni"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(
                fila_excel,
                2,
                comp["nombre"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(
                fila_excel,
                3,
                comp["regimen"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(
                fila_excel,
                4,
                comp["campo"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(
                fila_excel,
                5,
                comp["valor_propio"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(
                fila_excel,
                6,
                comp["valor_mef"].as_str().unwrap_or(""),
                fmt_fila,
            );
            let _ = hoja.write_string_with_format(fila_excel, 7, etiqueta_estado, fmt_fila);

            fila_excel += 1;
        }
    }

    let buffer = workbook
        .save_to_buffer()
        .map_err(|e| ApiError::InternalError(format!("Error al generar Excel: {}", e)))?;

    let fecha = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("comparacion_mef_{}.xlsx", fecha);

    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(buffer))
}
