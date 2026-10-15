pub mod handlers;
pub mod models;
pub mod repo;

use crate::common::middleware::JWT;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notificaciones")
            .route("/stream", web::get().to(handlers::stream_notificaciones))
            .service(
                web::scope("")
                    .wrap(JWT)
                    .route("", web::get().to(handlers::listar_notificaciones))
                    .route("/leer-todas", web::put().to(handlers::marcar_todas_leidas))
                    .route("/{id}/leer", web::put().to(handlers::marcar_leida)),
            ),
    );
}
