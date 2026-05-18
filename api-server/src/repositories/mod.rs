pub mod banco_repo;
pub mod gradoaca_repo;
pub mod personal_repo;
pub mod usuario_repo;
pub mod contacto_repo;
pub mod sindicato_repo;
pub mod documento_repo;
use std::sync::OnceLock;
pub fn get_db_key() -> &'static str {
    static DB_KEY: OnceLock<String> = OnceLock::new();
    DB_KEY.get_or_init(|| {
        std::env::var("DB_KEY").expect("La variable de entorno DB_KEY debe estar configurada")
    })
}
