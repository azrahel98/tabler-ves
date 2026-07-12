use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Clone)]
pub struct ResumenResponse {
    pub total: i64,
    pub activos: i64,
    pub por_regimen: Vec<DataResumen>,
    pub por_sexo: Vec<DataResumen>,
    pub por_sindicato: Vec<DataResumen>,
}
#[derive(Serialize, Clone)]
pub struct DataResumen {
    pub cantidad: i64,
    pub nombre: String,
}
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct BancosReport {
    pub id: i32,
    pub nombre: String,
}
#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
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
