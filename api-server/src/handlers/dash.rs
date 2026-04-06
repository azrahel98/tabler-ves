use crate::{
    AppState,
    handlers::personal::PerfilDni,
    middleware::error::ApiError,
    models::dash::{
        BancosReport, Cumpleaños, DataResumen, DbOrgani, Organigrama,
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
  cb.tipo_cuenta,
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
