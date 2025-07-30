use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::middleware::error::ApiError;

#[derive(Deserialize, Serialize)]
pub struct CumpleañosRequest {
    pub mes: i32,
    pub dia: i32,
}

impl CumpleañosRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.mes < 1 || self.mes > 12 {
            return Err(ApiError::BadRequest(
                1,
                "mes debe estar entre 1 y 12".into(),
            ));
        }
        if self.dia < 1 || self.dia > 31 {
            return Err(ApiError::BadRequest(
                2,
                "dia debe estar entre 1 y 31".into(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Cumpleaños {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad: Option<i32>,
}

#[derive(Serialize)]
pub struct ResumenResponse {
    pub total: i64,
    pub activos: i64,
    pub por_regimen: Vec<DataResumen>,
    pub por_sexo: Vec<DataResumen>,
    pub por_sindicato: Vec<DataResumen>,
}
#[derive(Serialize)]
pub struct DataResumen {
    pub cantidad: i64,
    pub nombre: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct AreaReport {
    pub dni: String,
    pub nombre: Option<String>,
    pub nacimiento: NaiveDate,
    pub edad: Option<i32>,
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

#[derive(FromRow, Deserialize)]
pub struct DbOrgani {
    pub id: i32,
    pub area: String,
    pub nombre: Option<String>,
    pub dni: Option<String>,
    pub nivel: Option<i32>,
}
