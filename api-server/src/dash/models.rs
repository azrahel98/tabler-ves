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
    pub condicion: Option<String>,
    pub subgerencias: Vec<Organigrama>,
}

#[derive(FromRow, Deserialize, Clone)]
pub struct DbOrgani {
    pub id: i32,
    pub area: String,
    pub nombre: Option<String>,
    pub dni: Option<String>,
    pub nivel: Option<i32>,
    pub condicion: Option<String>,
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

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Cumpleaños {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad: Option<i64>,
    pub avatar: Option<String>,
    pub regimen: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorialPaginado {
    pub items: Vec<serde_json::Value>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct TipoDocumentoItem {
    pub id: i32,
    pub nombre: Option<String>,
}

pub type ReporteDocumento = TipoDocumentoItem;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Alertas {
    Atiempo,
    AlLimite,
    Diciembre,
    Excepcional,
    Excedido,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Alerta70Anos {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad_actual: i64,
    pub area: String,
    pub cargo: String,
    pub regimen: Option<String>,
    pub plaza: Option<String>,
    pub avatar: Option<String>,
    pub estado: Option<Alertas>,
}
