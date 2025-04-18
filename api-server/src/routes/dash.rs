use crate::{handlers::dash::*, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dash")
            .wrap(JWT)
            .route("/cumpleaños", web::post().to(cumpleaños))
            .route("/area_report", web::post().to(personal_area_report))
            .route("/renuncias", web::post().to(renuncias_año))
            .route("/info", web::post().to(info)),
    );
}
