use serde::Serialize;

#[derive(Serialize)]
pub struct Persona {
    pub nombre: Option<String>,
    pub dni: String,
    pub estado: Option<String>,
    pub sexo: Option<String>,
}
