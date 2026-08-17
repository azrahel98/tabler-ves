use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
#[allow(non_snake_case)]
pub struct Cumpleaños {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad: Option<i64>,
    pub avatar: Option<String>,
    pub regimen: Option<String>,
}
#[derive(Serialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct ResumenResponse {
    pub total: i64,
    pub activos: i64,
    pub por_regimen: Vec<DataResumen>,
    pub por_sexo: Vec<DataResumen>,
    pub por_sindicato: Vec<DataResumen>,
}
#[derive(Serialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct DataResumen {
    pub cantidad: i64,
    pub nombre: String,
}
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
#[allow(non_snake_case)]
pub struct BancosReport {
    pub id: i32,
    pub nombre: String,
}
#[derive(Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct Organigrama {
    pub id: i32,
    pub area: String,
    pub jefe: Option<String>,
    pub dni: Option<String>,
    pub subgerencias: Vec<Organigrama>,
}
#[derive(FromRow, Deserialize, Clone)]
pub struct DbOrgani {
    pub id: i32,
    pub area: String,
    pub nombre: Option<String>,
    pub dni: Option<String>,
    pub nivel: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct ReporteRenuncias {
    pub id: i32,
    pub dni: String,
    pub nombre: Option<String>,
    pub fecha: Option<NaiveDate>,
    pub cargo: String,
    pub area: String,
    pub codigo: String,
    pub avatar: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
pub struct ReporteDocumento {
    pub id: i32,
    pub nombre: Option<String>,
    pub sigla: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
pub struct Alerta70Anos {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad_actual: i64,
    pub fecha_70_anos: NaiveDate,
    pub fecha_limite_mes: NaiveDate,
    pub fecha_extension_fin_ano: NaiveDate,
    pub dias_para_70: i64,
    pub dias_para_cese_mes: i64,
    pub dias_para_cese_extension: i64,
    pub estado_alerta: String,
    pub area: String,
    pub cargo: String,
    pub regimen: Option<String>,
    pub plaza: Option<String>,
    pub avatar: Option<String>,
}

