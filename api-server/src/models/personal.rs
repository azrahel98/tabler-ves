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
    pub id: i32,
    pub dni: String,
    pub doc_ingreso: Option<String>,
    pub numero_doc_ingreso: Option<String>,
    pub descrip_ingreso: Option<String>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub area: String,
    pub cargo: String,
    pub regimen: Option<String>,
    pub sueldo: Option<f64>,
    pub estado: String,
    pub doc_salida: Option<String>,
    pub descrip_salida: Option<String>,
    pub fecha_salida: Option<NaiveDate>,
    pub numero_doc_salida: Option<String>,
    pub sindicato: Option<String>,
}
