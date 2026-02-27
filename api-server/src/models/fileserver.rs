use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileServerMetadata {
    pub id: i32,
    pub documento_id: Option<i32>,
    pub dni_asociado: String,
    pub original_name: String,
    pub file_hash: String,
    pub extension: Option<String>,
    pub usuario_subida: String,
    pub fecha_subida: Option<chrono::NaiveDateTime>,
}
