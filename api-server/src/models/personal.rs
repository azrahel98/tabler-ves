use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize)]
pub struct Persona {
    pub nombre: Option<String>,
    pub dni: String,
    pub estado: Option<String>,
    pub sexo: Option<String>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Documento {
    pub id: Option<i32>,
    #[serde(rename = "tipoDocumento")]
    pub tipo: Option<String>,
    #[serde(rename = "numeroDocumento")]
    pub numero: Option<i32>,
    #[serde(rename = "añoDocumento")]
    pub año: Option<i32>,
    pub fecha: String,
    #[serde(rename = "fechaValida")]
    pub fecha_valida: Option<String>,
    pub conv: Option<i64>,
    pub descripcion: String,
    pub funcion: Option<i64>,
    pub sueldo: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DatosBancarios {
    pub id: i32,
    pub numero_cuenta: String,
    pub tipo_cuenta: Option<String>,
    pub cci: Option<String>,
    pub banco: String,
    pub estado: i8,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DatosBancariosResponse {
    pub numero_cuenta: String,
    pub tipo_cuenta: String,
    pub cci: Option<String>,
    pub banco: i32,
    pub estado: i8,
    pub dni: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GradoAcademico {
    pub id: i32,
    pub descripcion: Option<String>,
    pub abrv: String,
    pub dni: String,
}
