use sqlx::MySqlPool;
use crate::infrastructure::web::middleware::error::ApiError;
use crate::infrastructure::db::repositories::dash_repo;
use crate::infrastructure::web::models::dash::{
    Alerta70Anos, BancosReport, Cumpleaños, DataResumen, Organigrama, ReporteRenuncias, ResumenResponse, ReporteDocumento
};
use serde_json::{Value, json};


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



pub async fn reporte_historial(pool: &MySqlPool, dni: &str, key: &str) -> Result<Vec<Value>, ApiError> {
    dash_repo::get_historial(pool, dni, key).await
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

pub async fn reporte_documentos(pool: &MySqlPool) -> Result<Vec<ReporteDocumento>, ApiError> {
    dash_repo::get_documentos(pool).await
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
    regimen: String, 
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

pub async fn comparar_mef(pool: &MySqlPool, bytes_cas: Vec<u8>, bytes_otros: Vec<u8>) -> Result<Value, ApiError> {
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
        for (dni, datos) in mef_parsear_hoja(&range, "276/728")? {
            dict_mef.entry(dni).or_insert(datos);
        }
    }
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

pub async fn alerta_70_anos(
    pool: &MySqlPool,
    edad_min: Option<i32>,
) -> Result<Vec<Alerta70Anos>, ApiError> {
    dash_repo::get_alerta_70_anos(pool, edad_min).await
}

