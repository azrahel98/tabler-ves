use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, ts_rs::TS)]
#[ts(export)]
#[allow(non_snake_case)]
pub struct Usuario {
    pub id: i32,
    pub nickname: String,
    #[ts(skip)]
    pub pass: Option<String>,
    pub nombre: String,
    pub nivel: i32,
}
