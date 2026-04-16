use crate::{handlers::dash::*, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dash")
            .wrap(JWT)
            .route("/cumpleaños", web::post().to(cumpleaños))
            .route("/area_report", web::post().to(personal_area_report))
            .route("/renuncias", web::post().to(renuncias_año))
            .route("/banco_report", web::post().to(bancos_report))
            .route("/personal_activo", web::post().to(reporte_personal_activo))
            .route("/reporte_historia", web::post().to(reporte_historial))
            .route("/info", web::post().to(info))
            .route("/reporte_renuncias", web::post().to(report_renuncias))
            .route("/reporte_documentos", web::post().to(reporte_documentos))
            .route("/organigrama", web::post().to(organigrama))
            .route("/activos_por_distrito", web::post().to(activos_por_distrito))
            .route("/exportar_excel", web::post().to(exportar_excel))
            .route("/comparar_mef", web::post().to(comparar_mef))
            .route("/exportar_comparacion_mef", web::post().to(exportar_comparacion_mef))
            .route("/nuevos_trabajadores", web::post().to(nuevos_trabajadores))
            .route("/reporte_eventos", web::post().to(reporte_eventos)),
    );
}
