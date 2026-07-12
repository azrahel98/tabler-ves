use crate::infrastructure::web::{handlers::dash::*, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/dash")
            .wrap(JWT)
            .service(cumpleanos)
            .service(info)
            .service(personal_area_report)
            .service(renuncias_ano)
            .service(bancos_report)
            .service(reporte_personal_activo)
            .service(personal_activo_area)
            .service(reporte_historial)
            .service(organigrama)
            .service(report_renuncias)
            .service(reporte_documentos)
            .service(exportar_excel)
            .service(activos_por_distrito)
            .service(nuevos_trabajadores)
            .service(rangos_edad)
            .service(rangos_antiguedad)
            .service(reporte_eventos)
            .service(comparar_mef)
            .service(generar_mef)
    );
}
