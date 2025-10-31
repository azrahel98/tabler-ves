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
            .route("/reporte_legajos", web::post().to(report_legajos))
            .route("/reporte_renuncias", web::post().to(report_renuncias))
            .route("/organigrama", web::post().to(organigrama)),
    );
}
