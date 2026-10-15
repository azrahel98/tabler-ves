use crate::common::errors::ApiError;
use crate::dash::models::{
    AirhspCampoCambio, AirhspPersonaCambio, AirhspPersonaOmitida, AirhspResumen,
    Alerta70Anos, AplicarAirhspResponse, BancosReport, CatalogosValidacion, Cumpleaños,
    DataResumen, DatosMef, DatosSistema, HistorialPaginado, Organigrama, ReporteRenuncias,
    ResultadoEvaluacionPersona, ResumenResponse, TipoDocumentoItem,
};
use crate::dash::repo as dash_repo;
use serde_json::{Value, json};
use sqlx::MySqlPool;

pub async fn cumpleaños(pool: &MySqlPool) -> Result<Vec<Cumpleaños>, ApiError> {
    dash_repo::get_cumpleanos(pool).await
}

pub async fn info(pool: &MySqlPool) -> Result<ResumenResponse, ApiError> {
    let (total, activos) = dash_repo::get_info_total_activos(pool).await?;
    let por_regimen = dash_repo::get_info_por_regimen(pool).await?;
    let por_sexo = dash_repo::get_info_por_sexo(pool).await?;
    let por_sindicato = dash_repo::get_info_por_sindicato(pool).await?;

    Ok(ResumenResponse {
        total,
        activos,
        por_regimen,
        por_sexo,
        por_sindicato,
    })
}

pub async fn personal_area_report(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    dash_repo::get_personal_area_report(pool).await
}

pub async fn renuncias_ano(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    dash_repo::get_renuncias_ano(pool).await
}

pub async fn bancos_report(pool: &MySqlPool) -> Result<Vec<BancosReport>, ApiError> {
    dash_repo::get_bancos_report(pool).await
}

pub async fn reporte_personal_activo(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_reporte_personal_activo(pool).await
}

pub async fn personal_activo_area(pool: &MySqlPool, area_id: i32) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_personal_activo_area(pool, area_id).await
}

pub async fn personal_activo_sindicato(
    pool: &MySqlPool,
    sindicato_id: Option<i32>,
    sindicato_nombre: Option<&str>,
) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_personal_activo_sindicato(pool, sindicato_id, sindicato_nombre).await
}

pub async fn personal_activo_regimen(
    pool: &MySqlPool,
    regimen_id: Option<i32>,
    regimen_nombre: Option<&str>,
) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_personal_activo_regimen(pool, regimen_id, regimen_nombre).await
}

pub async fn reporte_historial(
    pool: &MySqlPool,
    dni: Option<&str>,
    page: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<HistorialPaginado, ApiError> {
    let limit = limit.unwrap_or(50).clamp(1, 200);
    let page = page.unwrap_or(1).max(1);
    let offset = offset.unwrap_or((page - 1) * limit);

    let (items, total) = dash_repo::get_historial(pool, dni, limit, offset).await?;
    let total_pages = if total == 0 {
        1
    } else {
        (total + limit - 1) / limit
    };

    Ok(HistorialPaginado {
        items,
        total,
        page,
        limit,
        total_pages,
    })
}

pub async fn organigrama(pool: &MySqlPool) -> Result<Vec<Organigrama>, ApiError> {
    let rows = dash_repo::get_db_organi(pool).await?;
    let mut organigrama: Vec<Organigrama> = Vec::new();

    // Nivel 0
    for row in &rows {
        if row.nivel.is_none() {
            let org = Organigrama {
                id: row.id,
                area: row.area.clone(),
                dni: row.dni.clone(),
                jefe: row.nombre.clone(),
                condicion: row.condicion.clone(),
                subgerencias: Vec::new(),
            };
            organigrama.push(org);
        }
    }

    // Nivel 1
    for row in &rows {
        if let Some(nivel) = row.nivel {
            for org in organigrama.iter_mut() {
                if org.id == nivel {
                    let sub = Organigrama {
                        id: row.id,
                        area: row.area.clone(),
                        dni: row.dni.clone(),
                        jefe: row.nombre.clone(),
                        condicion: row.condicion.clone(),
                        subgerencias: Vec::new(),
                    };
                    org.subgerencias.push(sub);
                }
            }
        }
    }

    // Nivel 2
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
                            condicion: row.condicion.clone(),
                            subgerencias: Vec::new(),
                        };
                        sub.subgerencias.push(new_sub);
                    }
                }
            }
        }
    }

    Ok(organigrama)
}

pub async fn report_renuncias(pool: &MySqlPool) -> Result<Vec<ReporteRenuncias>, ApiError> {
    dash_repo::get_renuncias(pool).await
}

pub async fn tipos_documento(pool: &MySqlPool) -> Result<Vec<TipoDocumentoItem>, ApiError> {
    dash_repo::get_tipos_documento(pool).await
}

pub async fn activos_por_distrito(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_activos_por_distrito(pool).await
}

// Logic for exportar_excel is placed here instead of the repo.
pub async fn exportar_excel(pool: &MySqlPool) -> Result<Vec<u8>, ApiError> {
    use rust_xlsxwriter::{Format, Workbook};
    let filas = dash_repo::get_exportar_excel_data(pool).await?;

    let mut workbook = Workbook::new();
    let hoja = workbook.add_worksheet();
    hoja.set_name("AIRHSP")
        .map_err(|e| ApiError::InternalError(format!("Error al nombrar hoja: {}", e)))?;
    let formato_cabecera = Format::new().set_bold();
    let cabeceras = [
        "UNIDAD_ORGANICA",
        "CODIGO_PUESTO_CPE",
        "ESTADO",
        "NUMERO_DOCUMENTO_IDENTIDAD",
        "APELLIDO_PATERNO",
        "APELLIDO_MATERNO",
        "NOMBRES",
        "CODIGO_SEXO",
        "DESC_SEXO",
        "FECHA_NACIMIENTO",
        "FECHA_INGRESO_PERSONAL",
        "REGIMEN_LABORAL",
        "CONDICION_LABORAL",
        "CODIGO_GRUPO_OCUPACIONAL",
        "GRUPO_OCUPACIONAL",
        "CODIGO_CARGO_ESTRUCTURAL",
        "CARGO_ESTRUCTURAL",
        "CARGO_FUNCIONAL",
        "ENTIDAD_FINANCIERA",
        "TIPO_CUENTA_FINANCIERA",
        "NUMERO_CUENTA_FINANCIERA",
        "CODIGO_CUENTA_INTERBANCARIA",
        "SUELDO",
        "SINDICATO",
    ];
    for (col, cabecera) in cabeceras.iter().enumerate() {
        hoja.write_string_with_format(0, col as u16, *cabecera, &formato_cabecera)
            .map_err(|e| ApiError::InternalError(format!("Error escribiendo cabecera: {}", e)))?;
    }
    use sqlx::Row;
    for (i, fila) in filas.iter().enumerate() {
        let num_fila = (i + 1) as u32;
        let dni: String = fila.get("dni");
        let apaterno: String = fila.get("apaterno");
        let amaterno: String = fila.get("amaterno");
        let nombre: String = fila.get("nombre");
        let sexo: Option<String> = fila.try_get("sexo").ok();
        let nacimiento: Option<chrono::NaiveDate> = fila.try_get("fecha_nacimiento").ok();
        let ingreso: Option<chrono::NaiveDate> = fila.try_get("ingreso").ok();
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
    workbook
        .save_to_buffer()
        .map_err(|e| ApiError::InternalError(format!("Error al generar Excel: {}", e)))
}

pub async fn nuevos_trabajadores(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_nuevos_trabajadores(pool).await
}

pub async fn rangos_edad(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    dash_repo::get_rangos_edad(pool).await
}

pub async fn rangos_antiguedad(pool: &MySqlPool) -> Result<Vec<DataResumen>, ApiError> {
    dash_repo::get_rangos_antiguedad(pool).await
}

pub async fn reporte_eventos(pool: &MySqlPool) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_reporte_eventos(pool).await
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
            continue;
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

pub fn parsear_archivos_mef(
    bytes_cas: &[u8],
    bytes_otros: &[u8],
) -> Result<std::collections::HashMap<String, DatosMef>, ApiError> {
    use calamine::{Reader, Xlsx, open_workbook_from_rs};
    use std::collections::HashMap;
    use std::io::Cursor;

    let mut dict_mef: HashMap<String, DatosMef> = HashMap::new();
    if !bytes_cas.is_empty() {
        let cursor = Cursor::new(bytes_cas);
        let mut wb: Xlsx<_> = open_workbook_from_rs(cursor)
            .map_err(|e| ApiError::InternalError(format!("Error abriendo Excel CAS: {}", e)))?;
        let sheet = wb
            .sheet_names()
            .first()
            .ok_or_else(|| ApiError::BadRequest("El archivo CAS no tiene hojas".to_string()))?
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
                ApiError::BadRequest("El archivo 276/728 no tiene hojas".to_string())
            })?
            .clone();
        let range = wb
            .worksheet_range(&sheet)
            .map_err(|e| ApiError::InternalError(format!("Error leyendo hoja 276/728: {}", e)))?;
        for (dni, datos) in mef_parsear_hoja(&range, "276/728")? {
            dict_mef.entry(dni).or_insert(datos);
        }
    }
    Ok(dict_mef)
}

pub async fn comparar_mef(
    pool: &MySqlPool,
    bytes_cas: Vec<u8>,
    bytes_otros: Vec<u8>,
) -> Result<Value, ApiError> {
    let dict_mef = parsear_archivos_mef(&bytes_cas, &bytes_otros)?;
    use sqlx::Row;
    let filas_bd = dash_repo::get_comparar_mef_data(pool).await?;

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
    let mut dnis_sistema: std::collections::HashSet<String> = std::collections::HashSet::new();

    for fila in &filas_bd {
        let dni: String = fila.get("dni");
        let apaterno: String = fila.get("apaterno");
        let amaterno: String = fila.get("amaterno");
        let nombre: String = fila.get("nombre");
        let nacimiento: Option<chrono::NaiveDate> = fila.try_get("fecha_nacimiento").ok();
        let ingreso: Option<chrono::NaiveDate> = fila.try_get("fecha_ingreso").ok();
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
            let num_mef_raw = mef.num_cuenta.trim();
            let es_cci_en_num =
                num_mef_raw.len() == 20 && num_mef_raw.chars().all(|c| c.is_ascii_digit());
            let cci_propio = mef.cci.trim().to_uppercase();
            let cci_efectivo_mef = if !cci_propio.is_empty() {
                cci_propio
            } else if es_cci_en_num {
                num_mef_raw.to_uppercase()
            } else {
                "".to_string()
            };

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
                let etiqueta = etiquetas[i];
                let vp = &valores_propios[i];
                let mut vm = valores_mef[i].clone();

                let igual = if etiqueta == "NUMERO_CUENTA_FINANCIERA" && es_cci_en_num {
                    num_mef_raw.eq_ignore_ascii_case(&num_cta_str)
                        || num_mef_raw.eq_ignore_ascii_case(&cci_str)
                } else if etiqueta == "CODIGO_CUENTA_INTERBANCARIA" {
                    vm = cci_efectivo_mef.clone();
                    vp.eq_ignore_ascii_case(&cci_efectivo_mef)
                        || (vp.is_empty() && cci_efectivo_mef.is_empty())
                } else {
                    vp == &vm || (vp.is_empty() && vm.is_empty())
                };

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
                    "campo": etiqueta,
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
    Ok(json!({
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
    }))
}

pub fn exportar_comparacion_mef(comparaciones: &[Value]) -> Result<Vec<u8>, ApiError> {
    use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook};
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
        let _ = hoja.set_column_width(2, 35);
        let _ = hoja.set_column_width(4, 30);
        let _ = hoja.set_column_width(5, 25);
        let _ = hoja.set_column_width(6, 25);
        let _ = hoja.set_column_width(7, 22);
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
    workbook
        .save_to_buffer()
        .map_err(|e| ApiError::InternalError(format!("Error al generar Excel: {}", e)))
}

pub async fn alerta_70_anos(pool: &MySqlPool) -> Result<Vec<Alerta70Anos>, ApiError> {
    dash_repo::get_alerta_70_anos(pool).await
}

pub fn mapear_tipo_cuenta_bd(tipo_airhsp: &str) -> Option<&'static str> {
    match tipo_airhsp.trim().to_uppercase().as_str() {
        "CUENTA DE AHORRO" | "AHORRO" => Some("Ahorro"),
        "CUENTA CORRIENTE" | "CORRIENTE" => Some("Corriente"),
        _ => None,
    }
}

pub fn evaluar_persona_airhsp(
    sistema: &DatosSistema,
    mef: &DatosMef,
    catalogos: &CatalogosValidacion,
) -> ResultadoEvaluacionPersona {
    let nombre_completo = format!(
        "{} {}, {}",
        sistema.apaterno.trim().to_uppercase(),
        sistema.amaterno.trim().to_uppercase(),
        sistema.nombre.trim().to_uppercase()
    );

    // 1. Coincidencia = mismo DNI y misma fecha de ingreso
    let fingreso_mef_res = chrono::NaiveDate::parse_from_str(mef.fingreso.trim(), "%d/%m/%Y");
    let coincide_ingreso = match (&sistema.fecha_ingreso, fingreso_mef_res) {
        (Some(f_sis), Ok(f_mef)) => *f_sis == f_mef,
        _ => false,
    };

    if !coincide_ingreso {
        return ResultadoEvaluacionPersona::OmitidoFechaIngreso(AirhspPersonaOmitida {
            dni: sistema.dni.clone(),
            nombre: nombre_completo,
            fecha_ingreso_sistema: sistema
                .fecha_ingreso
                .map(|d| d.format("%d/%m/%Y").to_string())
                .unwrap_or_default(),
            fecha_ingreso_airhsp: mef.fingreso.trim().to_string(),
        });
    }

    let mut campos: Vec<AirhspCampoCambio> = Vec::new();

    // 2. APELLIDO_PATERNO
    let apepat_mef = mef.apepat.trim().to_uppercase();
    let apepat_sis = sistema.apaterno.trim().to_uppercase();
    if !apepat_mef.is_empty() && apepat_mef != apepat_sis {
        campos.push(AirhspCampoCambio {
            campo: "APELLIDO_PATERNO".to_string(),
            antes: apepat_sis,
            despues: apepat_mef,
            error: None,
        });
    }

    // 3. APELLIDO_MATERNO
    let apemat_mef = mef.apemat.trim().to_uppercase();
    let apemat_sis = sistema.amaterno.trim().to_uppercase();
    if !apemat_mef.is_empty() && apemat_mef != apemat_sis {
        campos.push(AirhspCampoCambio {
            campo: "APELLIDO_MATERNO".to_string(),
            antes: apemat_sis,
            despues: apemat_mef,
            error: None,
        });
    }

    // 4. NOMBRES
    let nom_mef = mef.nom.trim().to_uppercase();
    let nom_sis = sistema.nombre.trim().to_uppercase();
    if !nom_mef.is_empty() && nom_mef != nom_sis {
        campos.push(AirhspCampoCambio {
            campo: "NOMBRES".to_string(),
            antes: nom_sis,
            despues: nom_mef,
            error: None,
        });
    }

    // 5. FECHA_NACIMIENTO
    let fnac_sis = sistema
        .fecha_nacimiento
        .map(|d| d.format("%d/%m/%Y").to_string())
        .unwrap_or_default();
    let fnac_mef_raw = mef.fnac.trim();
    if !fnac_mef_raw.is_empty() {
        match chrono::NaiveDate::parse_from_str(fnac_mef_raw, "%d/%m/%Y") {
            Ok(fnac_mef) => {
                let fnac_mef_fmt = fnac_mef.format("%d/%m/%Y").to_string();
                if fnac_mef_fmt != fnac_sis {
                    campos.push(AirhspCampoCambio {
                        campo: "FECHA_NACIMIENTO".to_string(),
                        antes: fnac_sis,
                        despues: fnac_mef_fmt,
                        error: None,
                    });
                }
            }
            Err(_) => {
                campos.push(AirhspCampoCambio {
                    campo: "FECHA_NACIMIENTO".to_string(),
                    antes: fnac_sis,
                    despues: fnac_mef_raw.to_string(),
                    error: Some(
                        "Formato de fecha de nacimiento inválido (se esperaba dd/mm/yyyy)"
                            .to_string(),
                    ),
                });
            }
        }
    }

    // 6. CODIGO_REGISTRO (plaza)
    let plaza_sis = sistema
        .plaza_id
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_uppercase();
    let plaza_mef = mef.codigo_registro.trim().to_uppercase();
    let mut plaza_final = plaza_sis.clone();

    if !plaza_mef.is_empty() {
        if plaza_mef.chars().count() != 6 {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_REGISTRO".to_string(),
                antes: plaza_sis.clone(),
                despues: plaza_mef.clone(),
                error: Some(
                    "El código de registro debe tener exactamente 6 caracteres".to_string(),
                ),
            });
        } else if plaza_mef != plaza_sis {
            let asignada_a_otro = match catalogos.plazas_ocupadas.get(&plaza_mef) {
                Some(otro_dni) => otro_dni != &sistema.dni,
                None => false,
            };

            if asignada_a_otro {
                let otro_dni = catalogos.plazas_ocupadas.get(&plaza_mef).unwrap();
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_REGISTRO".to_string(),
                    antes: plaza_sis.clone(),
                    despues: plaza_mef.clone(),
                    error: Some(format!(
                        "La plaza {} ya está asignada al DNI {}",
                        plaza_mef, otro_dni
                    )),
                });
            } else {
                let plaza_existe = catalogos.plazas_existentes.is_empty()
                    || catalogos.plazas_existentes.contains(&plaza_mef);
                if !plaza_existe {
                    let cargo_mef = mef.codigo_cargo_estructural.trim().to_uppercase();
                    let grupo_mef = mef.codigo_grupo_ocupacional.trim().to_uppercase();
                    let cargo_ok = !cargo_mef.is_empty()
                        && (catalogos.cargos_estructurales.is_empty()
                            || catalogos.cargos_estructurales.contains(&cargo_mef));
                    let grupo_ok = !grupo_mef.is_empty()
                        && (catalogos.grupos_ocupacionales.is_empty()
                            || catalogos.grupos_ocupacionales.contains(&grupo_mef));

                    if !cargo_ok || !grupo_ok {
                        campos.push(AirhspCampoCambio {
                            campo: "CODIGO_REGISTRO".to_string(),
                            antes: plaza_sis.clone(),
                            despues: plaza_mef.clone(),
                            error: Some(format!(
                                "La plaza {} no existe en el sistema y AIRHSP no trae cargo/grupo válidos para crearla",
                                plaza_mef
                            )),
                        });
                    } else {
                        campos.push(AirhspCampoCambio {
                            campo: "CODIGO_REGISTRO".to_string(),
                            antes: plaza_sis.clone(),
                            despues: plaza_mef.clone(),
                            error: None,
                        });
                        plaza_final = plaza_mef;
                    }
                } else {
                    campos.push(AirhspCampoCambio {
                        campo: "CODIGO_REGISTRO".to_string(),
                        antes: plaza_sis.clone(),
                        despues: plaza_mef.clone(),
                        error: None,
                    });
                    plaza_final = plaza_mef;
                }
            }
        }
    }

    let plaza_final_existe = plaza_final.is_empty()
        || catalogos.plazas_existentes.is_empty()
        || catalogos.plazas_existentes.contains(&plaza_final);

    let cargo_sis = sistema
        .codigo_cargo_estructural
        .trim()
        .to_uppercase();
    let cargo_mef = mef.codigo_cargo_estructural.trim().to_uppercase();
    let grupo_sis = sistema
        .codigo_grupo_ocupacional
        .trim()
        .to_uppercase();
    let grupo_mef = mef.codigo_grupo_ocupacional.trim().to_uppercase();

    if !plaza_final_existe {
        let cargo_valido = !cargo_mef.is_empty()
            && (catalogos.cargos_estructurales.is_empty()
                || catalogos.cargos_estructurales.contains(&cargo_mef));
        let grupo_valido = !grupo_mef.is_empty()
            && (catalogos.grupos_ocupacionales.is_empty()
                || catalogos.grupos_ocupacionales.contains(&grupo_mef));

        // 7. CODIGO_CARGO_ESTRUCTURAL
        if !cargo_valido {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_CARGO_ESTRUCTURAL".to_string(),
                antes: cargo_sis,
                despues: cargo_mef,
                error: Some(format!(
                    "La plaza {} no existe en el sistema y AIRHSP no trae cargo/grupo válidos para crearla",
                    plaza_final
                )),
            });
        } else if cargo_mef != cargo_sis {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_CARGO_ESTRUCTURAL".to_string(),
                antes: cargo_sis,
                despues: cargo_mef,
                error: None,
            });
        }

        // 8. CODIGO_GRUPO_OCUPACIONAL
        if !grupo_valido {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_GRUPO_OCUPACIONAL".to_string(),
                antes: grupo_sis,
                despues: grupo_mef,
                error: Some(format!(
                    "La plaza {} no existe en el sistema y AIRHSP no trae cargo/grupo válidos para crearla",
                    plaza_final
                )),
            });
        } else if grupo_mef != grupo_sis {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_GRUPO_OCUPACIONAL".to_string(),
                antes: grupo_sis,
                despues: grupo_mef,
                error: None,
            });
        }
    } else {
        // 7. CODIGO_CARGO_ESTRUCTURAL
        if !cargo_mef.is_empty() {
            let cargo_valido = catalogos.cargos_estructurales.is_empty()
                || catalogos.cargos_estructurales.contains(&cargo_mef);
            if !cargo_valido {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_CARGO_ESTRUCTURAL".to_string(),
                    antes: cargo_sis,
                    despues: cargo_mef,
                    error: Some("Código de cargo estructural no existe en el catálogo".to_string()),
                });
            } else if cargo_mef != cargo_sis {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_CARGO_ESTRUCTURAL".to_string(),
                    antes: cargo_sis,
                    despues: cargo_mef,
                    error: None,
                });
            }
        }

        // 8. CODIGO_GRUPO_OCUPACIONAL
        if !grupo_mef.is_empty() {
            let grupo_valido = catalogos.grupos_ocupacionales.is_empty()
                || catalogos.grupos_ocupacionales.contains(&grupo_mef);
            if !grupo_valido {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_GRUPO_OCUPACIONAL".to_string(),
                    antes: grupo_sis,
                    despues: grupo_mef,
                    error: Some("Código de grupo ocupacional no existe en el catálogo".to_string()),
                });
            } else if grupo_mef != grupo_sis {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_GRUPO_OCUPACIONAL".to_string(),
                    antes: grupo_sis,
                    despues: grupo_mef,
                    error: None,
                });
            }
        }
    }

    // 9. BANCA
    let antes_banco = sistema.banco.as_deref().unwrap_or("").trim().to_uppercase();
    let despues_banco = mef.banco.trim().to_uppercase();
    if !despues_banco.is_empty() {
        let banco_existe =
            catalogos.bancos.is_empty() || catalogos.bancos.contains(&despues_banco);
        if !banco_existe {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_ENTIDAD_FINANCIERA".to_string(),
                antes: antes_banco,
                despues: despues_banco,
                error: Some("Código de entidad financiera no existe en el catálogo".to_string()),
            });
        } else if despues_banco != antes_banco {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_ENTIDAD_FINANCIERA".to_string(),
                antes: antes_banco,
                despues: despues_banco,
                error: None,
            });
        }
    }

    let antes_tipo = sistema
        .tipo_cuenta
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_uppercase();
    let despues_tipo_raw = mef.tipo_cuenta.trim().to_uppercase();
    if !despues_tipo_raw.is_empty() {
        let (tipo_norm, err_tipo) = match mapear_tipo_cuenta_bd(&despues_tipo_raw) {
            Some("Ahorro") => ("CUENTA DE AHORRO".to_string(), None),
            Some("Corriente") => ("CUENTA CORRIENTE".to_string(), None),
            _ => (
                despues_tipo_raw.clone(),
                Some(format!(
                    "Tipo de cuenta '{}' no es compatible (solo se permite Ahorro o Corriente)",
                    despues_tipo_raw
                )),
            ),
        };

        if err_tipo.is_some() {
            campos.push(AirhspCampoCambio {
                campo: "TIPO_CUENTA_FINANCIERA".to_string(),
                antes: antes_tipo,
                despues: tipo_norm,
                error: err_tipo,
            });
        } else if tipo_norm != antes_tipo {
            campos.push(AirhspCampoCambio {
                campo: "TIPO_CUENTA_FINANCIERA".to_string(),
                antes: antes_tipo,
                despues: tipo_norm,
                error: None,
            });
        }
    }

    let antes_num = sistema
        .numero_cuenta
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_uppercase();
    let despues_num_raw = mef.num_cuenta.trim();
    let es_cci_en_num =
        despues_num_raw.len() == 20 && despues_num_raw.chars().all(|c| c.is_ascii_digit());
    let despues_num = despues_num_raw.to_uppercase();

    if !despues_num.is_empty() && !es_cci_en_num {
        let err_num = if let Some(owner_dni) = catalogos.cuentas_por_numero.get(&despues_num) {
            if owner_dni != &sistema.dni {
                Some("El número de cuenta ya pertenece a otra persona".to_string())
            } else {
                None
            }
        } else {
            None
        };

        if err_num.is_some() {
            campos.push(AirhspCampoCambio {
                campo: "NUMERO_CUENTA_FINANCIERA".to_string(),
                antes: antes_num,
                despues: despues_num.clone(),
                error: err_num,
            });
        } else if despues_num != antes_num {
            campos.push(AirhspCampoCambio {
                campo: "NUMERO_CUENTA_FINANCIERA".to_string(),
                antes: antes_num,
                despues: despues_num.clone(),
                error: None,
            });
        }
    }

    let antes_cci = sistema.cci.as_deref().unwrap_or("").trim().to_uppercase();
    let cci_propio = mef.cci.trim().to_uppercase();

    if es_cci_en_num {
        let cci_efectivo = despues_num.clone();
        if !cci_propio.is_empty() && cci_propio != cci_efectivo {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_CUENTA_INTERBANCARIA".to_string(),
                antes: antes_cci,
                despues: cci_propio,
                error: Some("AIRHSP trae dos CCI distintos".to_string()),
            });
        } else {
            let err_cci = if let Some(owner_dni) = catalogos.cuentas_por_cci.get(&cci_efectivo) {
                if owner_dni != &sistema.dni {
                    Some(
                        "El código de cuenta interbancaria (CCI) ya pertenece a otra persona"
                            .to_string(),
                    )
                } else {
                    None
                }
            } else {
                None
            };

            if err_cci.is_some() {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_CUENTA_INTERBANCARIA".to_string(),
                    antes: antes_cci,
                    despues: cci_efectivo,
                    error: err_cci,
                });
            } else if cci_efectivo != antes_cci {
                campos.push(AirhspCampoCambio {
                    campo: "CODIGO_CUENTA_INTERBANCARIA".to_string(),
                    antes: antes_cci,
                    despues: cci_efectivo,
                    error: None,
                });
            }
        }
    } else if !cci_propio.is_empty() {
        let err_cci = if let Some(owner_dni) = catalogos.cuentas_por_cci.get(&cci_propio) {
            if owner_dni != &sistema.dni {
                Some(
                    "El código de cuenta interbancaria (CCI) ya pertenece a otra persona"
                        .to_string(),
                )
            } else {
                None
            }
        } else {
            None
        };

        if err_cci.is_some() {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_CUENTA_INTERBANCARIA".to_string(),
                antes: antes_cci,
                despues: cci_propio,
                error: err_cci,
            });
        } else if cci_propio != antes_cci {
            campos.push(AirhspCampoCambio {
                campo: "CODIGO_CUENTA_INTERBANCARIA".to_string(),
                antes: antes_cci,
                despues: cci_propio,
                error: None,
            });
        }
    }

    // Validación de consistencia para datos bancarios
    let hay_campo_bancario = campos.iter().any(|c| {
        c.campo == "CODIGO_ENTIDAD_FINANCIERA"
            || c.campo == "TIPO_CUENTA_FINANCIERA"
            || c.campo == "NUMERO_CUENTA_FINANCIERA"
            || c.campo == "CODIGO_CUENTA_INTERBANCARIA"
    });

    if hay_campo_bancario {
        let banco_final = campos
            .iter()
            .find(|c| c.campo == "CODIGO_ENTIDAD_FINANCIERA")
            .map(|c| c.despues.clone())
            .or_else(|| sistema.banco.clone())
            .unwrap_or_default();
        let banco_invalido = banco_final.trim().is_empty()
            || (!catalogos.bancos.is_empty()
                && !catalogos.bancos.contains(&banco_final.trim().to_uppercase()));

        let tipo_final = campos
            .iter()
            .find(|c| c.campo == "TIPO_CUENTA_FINANCIERA")
            .map(|c| c.despues.clone())
            .or_else(|| sistema.tipo_cuenta.clone())
            .unwrap_or_default();
        let tipo_invalido = mapear_tipo_cuenta_bd(&tipo_final).is_none();

        let num_final = campos
            .iter()
            .find(|c| c.campo == "NUMERO_CUENTA_FINANCIERA")
            .map(|c| c.despues.clone())
            .or_else(|| sistema.numero_cuenta.clone())
            .unwrap_or_default();
        let num_invalido = num_final.trim().is_empty();

        let no_tiene_cuenta = sistema.cuenta_id.is_none()
            || sistema
                .numero_cuenta
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty();

        if banco_invalido || tipo_invalido || num_invalido {
            for c in campos.iter_mut() {
                if (c.campo == "CODIGO_ENTIDAD_FINANCIERA"
                    || c.campo == "TIPO_CUENTA_FINANCIERA"
                    || c.campo == "NUMERO_CUENTA_FINANCIERA"
                    || c.campo == "CODIGO_CUENTA_INTERBANCARIA")
                    && c.error.is_none()
                {
                    if no_tiene_cuenta && es_cci_en_num {
                        c.error = Some(
                            "AIRHSP solo trae CCI; se requiere el número de cuenta para crear la cuenta"
                                .to_string(),
                        );
                    } else if banco_invalido {
                        c.error = Some(
                            "No se puede registrar o actualizar la cuenta bancaria sin entidad financiera válida en el catálogo"
                                .to_string(),
                        );
                    } else if tipo_invalido {
                        c.error = Some(format!(
                            "Tipo de cuenta '{}' no es compatible (solo se permite Ahorro o Corriente)",
                            tipo_final
                        ));
                    } else if num_invalido {
                        c.error = Some(
                            "No se puede registrar o actualizar la cuenta bancaria con número de cuenta vacío"
                                .to_string(),
                        );
                    }
                }
            }
        }
    }


    if campos.is_empty() {
        ResultadoEvaluacionPersona::SinCambios
    } else {
        ResultadoEvaluacionPersona::Evaluado(AirhspPersonaCambio {
            dni: sistema.dni.clone(),
            nombre: nombre_completo,
            campos,
            error: None,
        })
    }
}

pub async fn aplicar_airhsp(
    pool: &MySqlPool,
    req: &actix_web::HttpRequest,
    bytes_cas: Vec<u8>,
    bytes_otros: Vec<u8>,
    confirmar: bool,
) -> Result<AplicarAirhspResponse, ApiError> {
    use sqlx::Row;

    let dict_mef = parsear_archivos_mef(&bytes_cas, &bytes_otros)?;
    let filas_bd = dash_repo::get_comparar_mef_data(pool).await?;
    let (catalogos, banco_id_por_codigo) = dash_repo::get_catalogos_airhsp(pool).await?;

    let mut resumen = AirhspResumen::default();
    let mut cambios: Vec<AirhspPersonaCambio> = Vec::new();
    let mut omitidos: Vec<AirhspPersonaOmitida> = Vec::new();

    for fila in &filas_bd {
        let dni: String = fila.get("dni");
        if let Some(mef) = dict_mef.get(&dni) {
            resumen.en_airhsp += 1;
            let sistema = dash_repo::row_to_datos_sistema(fila);

            match evaluar_persona_airhsp(&sistema, mef, &catalogos) {
                ResultadoEvaluacionPersona::OmitidoFechaIngreso(om) => {
                    resumen.omitidos_fecha_ingreso += 1;
                    omitidos.push(om);
                }
                ResultadoEvaluacionPersona::SinCambios => {
                    resumen.coincidencias += 1;
                    resumen.sin_cambios += 1;
                }
                ResultadoEvaluacionPersona::Evaluado(mut per_cambio) => {
                    resumen.coincidencias += 1;
                    resumen.con_cambios += 1;

                    let tiene_error_campo = per_cambio.campos.iter().any(|c| c.error.is_some());

                    if confirmar {
                        match dash_repo::aplicar_cambios_persona_tx(
                            pool,
                            &sistema,
                            &per_cambio,
                            mef,
                            &banco_id_por_codigo,
                        )
                        .await
                        {
                            Ok(campos_aplicados) => {
                                if !campos_aplicados.is_empty() {
                                    resumen.actualizados += 1;
                                    let campos_json: Vec<Value> = campos_aplicados
                                        .iter()
                                        .map(|c| {
                                            serde_json::json!({
                                                "campo": c.campo,
                                                "antes": c.antes,
                                                "despues": c.despues,
                                            })
                                        })
                                        .collect();

                                    let _ = crate::common::audit::registrar_historial(
                                        req,
                                        pool,
                                        "actualizacion airhsp",
                                        &sistema.dni,
                                        Some(serde_json::json!({ "campos": campos_json })),
                                    )
                                    .await;
                                }
                                if tiene_error_campo {
                                    resumen.errores += 1;
                                }
                            }
                            Err(e) => {
                                per_cambio.error = Some(e);
                                resumen.errores += 1;
                            }
                        }
                    } else if tiene_error_campo {
                        resumen.errores += 1;
                    }

                    cambios.push(per_cambio);
                }
            }
        }
    }

    Ok(AplicarAirhspResponse {
        aplicado: confirmar,
        resumen,
        cambios,
        omitidos,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn datos_sistema_base() -> DatosSistema {
        DatosSistema {
            vinculo_id: 10,
            dni: "12345678".to_string(),
            plaza_id: Some("000100".to_string()),
            nombre: "JUAN".to_string(),
            apaterno: "PEREZ".to_string(),
            amaterno: "LOPEZ".to_string(),
            fecha_nacimiento: Some(NaiveDate::from_ymd_opt(1990, 5, 15).unwrap()),
            fecha_ingreso: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
            banco: Some("001".to_string()),
            tipo_cuenta: Some("CUENTA DE AHORRO".to_string()),
            numero_cuenta: Some("111122223333".to_string()),
            cci: Some("00111122223333444455".to_string()),
            codigo_cargo_estructural: "C001".to_string(),
            codigo_grupo_ocupacional: "G1".to_string(),
            cuenta_id: Some(5),
        }
    }

    fn datos_mef_base() -> DatosMef {
        DatosMef {
            codigo_registro: "000100".to_string(),
            codigo_puesto: "".to_string(),
            apepat: "PEREZ".to_string(),
            apemat: "LOPEZ".to_string(),
            nom: "JUAN".to_string(),
            fnac: "15/05/1990".to_string(),
            fingreso: "01/01/2022".to_string(),
            banco: "001".to_string(),
            tipo_cuenta: "CUENTA DE AHORRO".to_string(),
            num_cuenta: "111122223333".to_string(),
            cci: "00111122223333444455".to_string(),
            regimen: "CAS".to_string(),
            codigo_grupo_ocupacional: "G1".to_string(),
            codigo_cargo_estructural: "C001".to_string(),
        }
    }

    #[test]
    fn test_evaluar_coincide_fecha_genera_cambios() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.nom = "JUAN CARLOS".to_string();

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);

        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                assert_eq!(cambio.dni, "12345678");
                assert_eq!(cambio.campos.len(), 1);
                assert_eq!(cambio.campos[0].campo, "NOMBRES");
                assert_eq!(cambio.campos[0].antes, "JUAN");
                assert_eq!(cambio.campos[0].despues, "JUAN CARLOS");
                assert!(cambio.campos[0].error.is_none());
            }
            _ => panic!("Se esperaba Evaluado con cambios"),
        }
    }

    #[test]
    fn test_evaluar_fecha_distinta_omitido() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.fingreso = "02/01/2022".to_string(); // Fecha diferente

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);

        match res {
            ResultadoEvaluacionPersona::OmitidoFechaIngreso(omitido) => {
                assert_eq!(omitido.dni, "12345678");
                assert_eq!(omitido.fecha_ingreso_sistema, "01/01/2022");
                assert_eq!(omitido.fecha_ingreso_airhsp, "02/01/2022");
            }
            _ => panic!("Se esperaba OmitidoFechaIngreso"),
        }
    }

    #[test]
    fn test_evaluar_valor_vacio_en_airhsp_no_genera_cambio() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        // Ponemos campos vacíos en AIRHSP
        mef.apepat = "".to_string();
        mef.apemat = "   ".to_string();
        mef.nom = "".to_string();
        mef.fnac = "".to_string();
        mef.banco = "".to_string();
        mef.tipo_cuenta = "".to_string();
        mef.num_cuenta = "".to_string();
        mef.cci = "".to_string();

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);

        assert_eq!(res, ResultadoEvaluacionPersona::SinCambios);
    }

    #[test]
    fn test_mapear_tipo_cuenta() {
        assert_eq!(mapear_tipo_cuenta_bd("CUENTA DE AHORRO"), Some("Ahorro"));
        assert_eq!(mapear_tipo_cuenta_bd("ahorro"), Some("Ahorro"));
        assert_eq!(mapear_tipo_cuenta_bd("CUENTA CORRIENTE"), Some("Corriente"));
        assert_eq!(mapear_tipo_cuenta_bd("corriente"), Some("Corriente"));
        assert_eq!(mapear_tipo_cuenta_bd("OTRA CUENTA"), None);

        // En evaluar_persona_airhsp con tipo no reconocido genera error de campo
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.tipo_cuenta = "FONDOS MUTUOS".to_string();

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo = cambio.campos.iter().find(|c| c.campo == "TIPO_CUENTA_FINANCIERA");
                assert!(campo.is_some());
                assert!(campo.unwrap().error.is_some());
            }
            _ => panic!("Se esperaba Evaluado con error en tipo de cuenta"),
        }
    }

    #[test]
    fn test_evaluar_fecha_nacimiento_invalida_genera_error_de_campo() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.fnac = "99/99/9999".to_string(); // Fecha inválida

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);

        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_fnac = cambio.campos.iter().find(|c| c.campo == "FECHA_NACIMIENTO");
                assert!(campo_fnac.is_some());
                let c = campo_fnac.unwrap();
                assert!(c.error.is_some());
                assert_eq!(c.despues, "99/99/9999");
            }
            _ => panic!("Se esperaba Evaluado con error en FECHA_NACIMIENTO"),
        }
    }

    #[test]
    fn test_evaluar_plaza_ya_ocupada_por_otro_dni() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.codigo_registro = "000200".to_string(); // Intenta asignar plaza 000200
        mef.codigo_cargo_estructural = "C002".to_string();

        let mut catalogos = CatalogosValidacion::default();
        catalogos.cargos_estructurales.insert("C001".to_string());
        catalogos.cargos_estructurales.insert("C002".to_string());
        catalogos.plazas_existentes.insert("000100".to_string());
        catalogos.plazas_existentes.insert("000200".to_string());
        // La plaza 000200 está ocupada por el DNI 87654321
        catalogos
            .plazas_ocupadas
            .insert("000200".to_string(), "87654321".to_string());

        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_reg = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_REGISTRO")
                    .expect("Debe existir campo CODIGO_REGISTRO");
                assert_eq!(
                    campo_reg.error.as_deref(),
                    Some("La plaza 000200 ya está asignada al DNI 87654321")
                );

                // Cargo estructural debe evaluarse contra la plaza actual del sistema (C001 -> C002)
                let campo_cargo = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_CARGO_ESTRUCTURAL")
                    .expect("Debe existir campo CODIGO_CARGO_ESTRUCTURAL evaluado contra plaza actual");
                assert_eq!(campo_cargo.antes, "C001");
                assert_eq!(campo_cargo.despues, "C002");
                assert!(campo_cargo.error.is_none());
            }
            _ => panic!("Se esperaba Evaluado con error en CODIGO_REGISTRO"),
        }
    }

    #[test]
    fn test_evaluar_banca_tipo_incompatible() {
        let sistema = datos_sistema_base();
        let mut mef = datos_mef_base();
        mef.tipo_cuenta = "CUENTA SUELDO".to_string();

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_tipo = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "TIPO_CUENTA_FINANCIERA")
                    .expect("Debe existir campo TIPO_CUENTA_FINANCIERA");
                assert!(campo_tipo.error.is_some());
                assert!(
                    campo_tipo
                        .error
                        .as_ref()
                        .unwrap()
                        .contains("no es compatible")
                );
            }
            _ => panic!("Se esperaba Evaluado con error en tipo de cuenta incompatible"),
        }
    }

    #[test]
    fn test_evaluar_banca_numero_vacio_genera_error() {
        let mut sistema = datos_sistema_base();
        sistema.numero_cuenta = None;
        sistema.banco = None;
        sistema.tipo_cuenta = None;
        sistema.cuenta_id = None;

        let mut mef = datos_mef_base();
        mef.banco = "001".to_string();
        mef.tipo_cuenta = "CUENTA DE AHORRO".to_string();
        mef.num_cuenta = "".to_string(); // Número vacío
        mef.cci = "00111122223333444455".to_string();

        let mut catalogos = CatalogosValidacion::default();
        catalogos.bancos.insert("001".to_string());

        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_banco = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_ENTIDAD_FINANCIERA");
                assert!(campo_banco.is_some());
                assert!(
                    campo_banco
                        .unwrap()
                        .error
                        .as_ref()
                        .unwrap()
                        .contains("número de cuenta")
                );
            }
            _ => panic!("Se esperaba Evaluado con error de cuenta bancaria"),
        }
    }

    #[test]
    fn test_evaluar_plaza_inexistente_con_cargo_grupo_validos_genera_cambios() {
        let mut sistema = datos_sistema_base();
        sistema.plaza_id = Some("000722".to_string());
        sistema.codigo_cargo_estructural = "".to_string();
        sistema.codigo_grupo_ocupacional = "".to_string();

        let mut mef = datos_mef_base();
        mef.codigo_registro = "000722".to_string();
        mef.codigo_cargo_estructural = "C001".to_string();
        mef.codigo_grupo_ocupacional = "G1".to_string();

        let mut catalogos = CatalogosValidacion::default();
        catalogos.plazas_existentes.insert("000100".to_string()); // 000722 no existe en plaza
        catalogos.cargos_estructurales.insert("C001".to_string());
        catalogos.grupos_ocupacionales.insert("G1".to_string());

        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_cargo = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_CARGO_ESTRUCTURAL")
                    .expect("Debe existir CODIGO_CARGO_ESTRUCTURAL");
                assert_eq!(campo_cargo.antes, "");
                assert_eq!(campo_cargo.despues, "C001");
                assert!(campo_cargo.error.is_none());

                let campo_grupo = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_GRUPO_OCUPACIONAL")
                    .expect("Debe existir CODIGO_GRUPO_OCUPACIONAL");
                assert_eq!(campo_grupo.antes, "");
                assert_eq!(campo_grupo.despues, "G1");
                assert!(campo_grupo.error.is_none());
            }
            _ => panic!("Se esperaba Evaluado con cambios aplicables"),
        }
    }

    #[test]
    fn test_evaluar_plaza_inexistente_sin_cargo_genera_error() {
        let mut sistema = datos_sistema_base();
        sistema.plaza_id = Some("000722".to_string());
        sistema.codigo_cargo_estructural = "".to_string();
        sistema.codigo_grupo_ocupacional = "".to_string();

        let mut mef = datos_mef_base();
        mef.codigo_registro = "000722".to_string();
        mef.codigo_cargo_estructural = "".to_string(); // Falta cargo
        mef.codigo_grupo_ocupacional = "G1".to_string();

        let mut catalogos = CatalogosValidacion::default();
        catalogos.plazas_existentes.insert("000100".to_string());
        catalogos.grupos_ocupacionales.insert("G1".to_string());

        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let campo_cargo = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_CARGO_ESTRUCTURAL")
                    .expect("Debe existir CODIGO_CARGO_ESTRUCTURAL con error");
                assert_eq!(
                    campo_cargo.error.as_deref(),
                    Some("La plaza 000722 no existe en el sistema y AIRHSP no trae cargo/grupo válidos para crearla")
                );
            }
            _ => panic!("Se esperaba Evaluado con error en cargo"),
        }
    }

    #[test]
    fn test_evaluar_numero_cuenta_20_digitos_tratado_como_cci_sin_cambio_en_numero() {
        let mut sistema = datos_sistema_base();
        sistema.numero_cuenta = Some("12345678901234".to_string());
        sistema.cci = Some("00219400000000000000".to_string());

        let mut mef = datos_mef_base();
        mef.num_cuenta = "00219412078940509790".to_string(); // 20 dígitos: CCI de BCP
        mef.cci = "".to_string(); // Vacío en AIRHSP

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                // NO se debe generar cambio en NUMERO_CUENTA_FINANCIERA
                let tiene_cambio_num = cambio
                    .campos
                    .iter()
                    .any(|c| c.campo == "NUMERO_CUENTA_FINANCIERA");
                assert!(!tiene_cambio_num, "No debe generarse cambio en NUMERO_CUENTA_FINANCIERA");

                // Se genera cambio en CODIGO_CUENTA_INTERBANCARIA usando el valor de 20 dígitos
                let campo_cci = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_CUENTA_INTERBANCARIA")
                    .expect("Debe existir cambio en CODIGO_CUENTA_INTERBANCARIA");
                assert_eq!(campo_cci.antes, "00219400000000000000");
                assert_eq!(campo_cci.despues, "00219412078940509790");
                assert!(campo_cci.error.is_none());
            }
            _ => panic!("Se esperaba Evaluado con cambio en CCI y sin cambio en número"),
        }
    }

    #[test]
    fn test_evaluar_dos_cci_distintos_genera_error() {
        let mut sistema = datos_sistema_base();
        sistema.numero_cuenta = Some("12345678901234".to_string());
        sistema.cci = Some("00219400000000000000".to_string());

        let mut mef = datos_mef_base();
        mef.num_cuenta = "00219412078940509790".to_string(); // 20 dígitos
        mef.cci = "00219499999999999999".to_string(); // Otro CCI distinto

        let catalogos = CatalogosValidacion::default();
        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                let tiene_cambio_num = cambio
                    .campos
                    .iter()
                    .any(|c| c.campo == "NUMERO_CUENTA_FINANCIERA");
                assert!(!tiene_cambio_num, "No debe generarse cambio en NUMERO_CUENTA_FINANCIERA");

                let campo_cci = cambio
                    .campos
                    .iter()
                    .find(|c| c.campo == "CODIGO_CUENTA_INTERBANCARIA")
                    .expect("Debe existir CODIGO_CUENTA_INTERBANCARIA con error");
                assert_eq!(
                    campo_cci.error.as_deref(),
                    Some("AIRHSP trae dos CCI distintos")
                );
            }
            _ => panic!("Se esperaba Evaluado con error de dos CCI distintos"),
        }
    }

    #[test]
    fn test_evaluar_persona_sin_cuenta_solo_cci_genera_error() {
        let mut sistema = datos_sistema_base();
        sistema.numero_cuenta = None;
        sistema.banco = None;
        sistema.tipo_cuenta = None;
        sistema.cuenta_id = None;
        sistema.cci = None;

        let mut mef = datos_mef_base();
        mef.banco = "001".to_string();
        mef.tipo_cuenta = "CUENTA DE AHORRO".to_string();
        mef.num_cuenta = "00219412078940509790".to_string(); // 20 dígitos
        mef.cci = "".to_string();

        let mut catalogos = CatalogosValidacion::default();
        catalogos.bancos.insert("001".to_string());

        let res = evaluar_persona_airhsp(&sistema, &mef, &catalogos);
        match res {
            ResultadoEvaluacionPersona::Evaluado(cambio) => {
                for c in &cambio.campos {
                    if c.campo == "CODIGO_CUENTA_INTERBANCARIA" || c.campo == "CODIGO_ENTIDAD_FINANCIERA" {
                        assert_eq!(
                            c.error.as_deref(),
                            Some("AIRHSP solo trae CCI; se requiere el número de cuenta para crear la cuenta")
                        );
                    }
                }
            }
            _ => panic!("Se esperaba Evaluado con error de falta de número de cuenta"),
        }
    }
}

