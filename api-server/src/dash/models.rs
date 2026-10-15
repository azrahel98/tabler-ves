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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatosMef {
    pub codigo_registro: String,
    #[allow(dead_code)]
    pub codigo_puesto: String,
    pub apepat: String,
    pub apemat: String,
    pub nom: String,
    pub fnac: String,
    pub fingreso: String,
    pub banco: String,
    pub tipo_cuenta: String,
    pub num_cuenta: String,
    pub cci: String,
    pub regimen: String,
    pub codigo_grupo_ocupacional: String,
    pub codigo_cargo_estructural: String,
}

#[derive(Debug, Clone, Default)]
pub struct DatosSistema {
    pub vinculo_id: i32,
    pub dni: String,
    pub plaza_id: Option<String>,
    pub nombre: String,
    pub apaterno: String,
    pub amaterno: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub banco: Option<String>,
    pub tipo_cuenta: Option<String>,
    pub numero_cuenta: Option<String>,
    pub cci: Option<String>,
    pub codigo_cargo_estructural: String,
    pub codigo_grupo_ocupacional: String,
    pub cuenta_id: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct CatalogosValidacion {
    pub bancos: std::collections::HashSet<String>,
    pub cargos_estructurales: std::collections::HashSet<String>,
    pub grupos_ocupacionales: std::collections::HashSet<String>,
    pub plazas_existentes: std::collections::HashSet<String>,
    pub plazas_ocupadas: std::collections::HashMap<String, String>,
    pub cuentas_por_numero: std::collections::HashMap<String, String>,
    pub cuentas_por_cci: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AirhspCampoCambio {
    pub campo: String,
    pub antes: String,
    pub despues: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AirhspPersonaCambio {
    pub dni: String,
    pub nombre: String,
    pub campos: Vec<AirhspCampoCambio>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AirhspPersonaOmitida {
    pub dni: String,
    pub nombre: String,
    pub fecha_ingreso_sistema: String,
    pub fecha_ingreso_airhsp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct AirhspResumen {
    pub en_airhsp: usize,
    pub coincidencias: usize,
    pub con_cambios: usize,
    pub sin_cambios: usize,
    pub omitidos_fecha_ingreso: usize,
    pub errores: usize,
    pub actualizados: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AplicarAirhspResponse {
    pub aplicado: bool,
    pub resumen: AirhspResumen,
    pub cambios: Vec<AirhspPersonaCambio>,
    pub omitidos: Vec<AirhspPersonaOmitida>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultadoEvaluacionPersona {
    OmitidoFechaIngreso(AirhspPersonaOmitida),
    Evaluado(AirhspPersonaCambio),
    SinCambios,
}
