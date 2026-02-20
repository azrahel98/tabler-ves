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
#[derive(Serialize, Deserialize)]
pub struct Vinculos {
    pub id: i32,
    pub dni: String,
    pub doc_ingreso: Option<String>,
    pub numero_doc_ingreso: Option<String>,
    pub descrip_ingreso: Option<String>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub area: Option<String>,
    pub cargo: String,
    pub regimen: Option<String>,
    pub sueldo: Option<f64>,
    pub codigo: Option<String>,
    pub cargo_estructural: Option<String>,
    pub grupo_ocupacional: Option<String>,
    pub estado: String,
    pub doc_salida: Option<String>,
    pub descrip_salida: Option<String>,
    pub fecha_salida: Option<NaiveDate>,
    pub numero_doc_salida: Option<String>,
    pub sindicato: Option<String>,
    pub tipo_evento: Option<String>,
    pub estado_evento: Option<String>,
    pub doc_evento_tipo: Option<String>,
    pub numero_doc_evento: Option<i32>,
    pub fecha_evento: Option<NaiveDate>,
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
    pub dni: String,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DocumentoSindicato {
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
    pub descripcion: String,
    pub sindicato: i32,
    pub id_vinculo: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dni: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LegajoPersonal {
    pub id: i32,
    pub persona: String,
    pub dni: Option<String>,
    pub fecha: Option<String>,
    pub estado: Option<String>,
    pub descrip: Option<String>,
    pub nuevo: i32,
    pub user: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AsistenciaVw {
    pub dni: String,
    pub hora: Option<String>,
    pub fecha: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactoEmergencia {
    pub persona_dni: String,
    pub nombre: String,
    pub relacion: String,
    pub telefono: Option<String>,
}
