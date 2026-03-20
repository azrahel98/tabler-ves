use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use validator::Validate;
use validator::ValidationError;

/// Valida que el DNI tenga exactamente 8 dígitos numéricos.
pub fn es_dni_valido(dni: &str) -> Result<(), ValidationError> {
    if dni.len() == 8 && dni.chars().all(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        let mut err = ValidationError::new("dni_invalido");
        err.message = Some("Debe tener exactamente 8 dígitos numéricos".into());
        Err(err)
    }
}

#[derive(Serialize)]
pub struct Persona {
    pub nombre: Option<String>,
    pub dni: String,
    pub estado: Option<String>,
    pub sexo: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Perfil {
    #[validate(custom(function = "es_dni_valido"))]
    pub dni: String,
    pub nombre: Option<String>,
    pub telf: Option<String>,
    pub direccion: Option<String>,
    pub email: Option<String>,
    pub ruc: Option<String>,
    pub nacimiento: NaiveDate,
    pub sexo: Option<String>,
    pub region: Option<String>,
    pub distrito: Option<String>,
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
    pub id_evento: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Documento {
    pub id: Option<i32>,
    #[serde(rename = "tipoDocumento")]
    pub tipo: Option<String>,
    #[serde(rename = "numeroDocumento")]
    pub numero: Option<i32>,
    #[serde(rename = "añoDocumento")]
    pub año: Option<i32>,
    #[validate(length(min = 1, message = "La fecha es requerida"))]
    pub fecha: String,
    #[serde(rename = "fechaValida")]
    pub fecha_valida: Option<String>,
    pub conv: Option<i64>,
    #[validate(length(min = 1, message = "La descripción es requerida"))]
    pub descripcion: String,
    pub funcion: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct DatosBancarios {
    pub id: i32,
    #[validate(length(
        min = 6,
        max = 30,
        message = "Número de cuenta inválido (6-30 caracteres)"
    ))]
    pub numero_cuenta: String,
    pub tipo_cuenta: Option<String>,
    pub cci: Option<String>,
    pub banco: String,
    pub dni: String,
    pub estado: i8,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct DatosBancariosResponse {
    #[validate(length(
        min = 6,
        max = 30,
        message = "Número de cuenta inválido (6-30 caracteres)"
    ))]
    pub numero_cuenta: String,
    #[validate(length(min = 1, message = "El tipo de cuenta es requerido"))]
    pub tipo_cuenta: String,
    pub cci: Option<String>,
    pub banco: i32,
    pub estado: i8,
    #[validate(custom(function = "es_dni_valido"))]
    pub dni: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct GradoAcademico {
    pub id: i32,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
    pub profesion: String,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
    pub universidad: String,
    pub colegiatura: Option<String>,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
    pub nivel_academico: String,
    #[validate(length(min = 1, message = "La abreviatura es requerida"))]
    pub abrv: String,
    #[validate(custom(function = "es_dni_valido"))]
    pub dni: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VinculosSindicato {
    pub id_vinculo: i32,
    pub dni: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct DocumentoSindicato {
    pub id: Option<i32>,
    #[serde(rename = "tipoDocumento")]
    pub tipo: Option<String>,
    #[serde(rename = "numeroDocumento")]
    pub numero: Option<i32>,
    #[serde(rename = "añoDocumento")]
    pub año: Option<i32>,
    #[validate(length(min = 1, message = "La fecha es requerida"))]
    pub fecha: String,
    #[serde(rename = "fechaValida")]
    pub fecha_valida: Option<String>,
    #[validate(length(min = 1, message = "La descripción es requerida"))]
    pub descripcion: String,
    pub sindicato: i32,
    pub vinculos: Vec<VinculosSindicato>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct LegajoPersonal {
    pub id: i32,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
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

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct ContactoEmergencia {
    #[validate(custom(function = "es_dni_valido"))]
    pub persona_dni: String,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
    pub nombre: String,
    #[validate(length(min = 2, message = "Mínimo 2 caracteres"))]
    pub relacion: String,
    pub telefono: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PerfilInput {
    #[validate(custom(function = "es_dni_valido"))]
    pub dni: String,
    #[validate(length(min = 1, message = "El apellido materno es requerido"))]
    pub amaterno: String,
    #[validate(length(min = 1, message = "El apellido paterno es requerido"))]
    pub apaterno: String,
    #[validate(length(min = 1, message = "El nombre es requerido"))]
    pub nombre: String,
    pub telf: Option<String>,
    pub direccion: Option<String>,
    pub email: Option<String>,
    pub ruc: Option<String>,
    pub nacimiento: NaiveDate,
    pub sexo: Option<String>,
    pub region: Option<String>,
    pub distrito: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NuevoVinculo {
    #[validate(nested)]
    pub personal: PerfilInput,
    #[validate(length(min = 1, message = "El código de plaza es requerido"))]
    pub airshp: String,
    #[validate(nested)]
    pub documento: Documento,
    #[validate(range(min = 1, message = "ID de régimen inválido"))]
    pub regimen: i32,
    #[validate(range(min = 1, message = "ID de cargo inválido"))]
    pub cargo: i32,
    #[validate(range(min = 1, message = "ID de área inválido"))]
    pub area: i32,
    #[validate(range(min = 0.0, message = "El sueldo debe ser mayor o igual a 0"))]
    pub sueldo: f64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EventoVinculoPayload {
    pub id: Option<i32>,
    #[validate(range(min = 1, message = "ID de vínculo inválido"))]
    pub vinculo_id: i32,
    #[validate(length(min = 1, message = "El tipo de evento es requerido"))]
    pub tipo_evento: String,
    pub nueva_area_id: Option<i32>,
    pub documento_inicio: Option<Documento>,
    pub documento_salida: Option<Documento>,
    pub estado: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteEventoVinculoPayload {
    #[validate(range(min = 1, message = "ID de evento inválido"))]
    pub id: i32,
}
