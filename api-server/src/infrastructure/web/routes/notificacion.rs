use crate::infrastructure::web::handlers::notificacion::*;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notificaciones")
            .route("/stream", web::get().to(stream_notificaciones))
            .route("", web::get().to(listar_notificaciones))
            .route("/leer-todas", web::put().to(marcar_todas_leidas))
            .route("/{id}/leer", web::put().to(marcar_leida)),
    );
}
