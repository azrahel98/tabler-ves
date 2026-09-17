pub mod handlers;
pub mod models;
pub mod repo;
pub mod service;

use crate::common::middleware::JWT;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/dash")
            .wrap(JWT)
            .service(handlers::cumpleanos)
            .service(handlers::info)
            .service(handlers::personal_area_report)
            .service(handlers::renuncias_ano)
            .service(handlers::bancos_report)
            .service(handlers::reporte_personal_activo)
            .service(handlers::personal_activo_area)
            .service(handlers::personal_activo_sindicato)
            .service(handlers::personal_activo_regimen)
            .service(handlers::reporte_historial)
            .service(handlers::organigrama)
            .service(handlers::report_renuncias)
            .service(handlers::reporte_documentos)
            .service(handlers::tipos_documento)
            .service(handlers::exportar_excel)
            .service(handlers::activos_por_distrito)
            .service(handlers::nuevos_trabajadores)
            .service(handlers::rangos_edad)
            .service(handlers::rangos_antiguedad)
            .service(handlers::reporte_eventos)
            .service(handlers::comparar_mef)
            .service(handlers::generar_mef)
            .service(handlers::alerta_70_anos),
    );
}
