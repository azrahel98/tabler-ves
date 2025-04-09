use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Persona {
    pub nombre: Option<String>,
    pub dni: String,
    pub estado: Option<String>,
    pub sexo: Option<String>,
}

#[derive(Serialize)]
pub struct Perfil {
    pub dni: String,
    pub nombre: Option<String>,
    pub telf: Option<String>,
    pub direccion: Option<String>,
    pub email: Option<String>,
    pub ruc: Option<String>,
    pub nacimiento: NaiveDate,
    pub sexo: Option<String>,
}

#[derive(Serialize)]
pub struct Vinculos {
    pub int: Option<i32>,
    pub dni: Option<String>,
    pub doc_ingreso: Option<String>,
    pub numero_doc_i: Option<String>,
    pub fecha_ingreso: Option<String>,
    pub area: Option<String>,
    pub cargo: Option<String>,
    pub regimen: Option<String>,
    pub sueldo: Option<f64>,
    pub estado: Option<String>,
    pub doc_salida: Option<String>,
    pub descrip_salida: Option<String>,
    pub fecha_salida: Option<String>,
    pub numero_doc_salida: Option<String>,
    pub nacimiento: NaiveDate,
    pub sindicato: Option<String>,
}
