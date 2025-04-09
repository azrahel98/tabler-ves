use crate::{
    handlers::personal::{buscar_por_nombre, perfil_por_dni},
    middleware::check::JWT,
};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal")
            .wrap(JWT)
            .route("/buscar", web::post().to(buscar_por_nombre))
            .route("/por_dni", web::post().to(perfil_por_dni)),
    );
}
