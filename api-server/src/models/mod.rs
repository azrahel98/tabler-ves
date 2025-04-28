pub mod dash;
pub mod login;
pub mod personal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Historial {
    pub id: i32,
    pub idusuario: i32,
    pub operacion: String,
    pub detalle: Option<String>,
    pub fecha: chrono::NaiveDateTime,
}
