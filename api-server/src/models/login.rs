use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Usuario {
    pub id: i32,
    pub nickname: String,
    pub pass: Option<String>,
    pub nombre: String,
    pub nivel: i32,
}
