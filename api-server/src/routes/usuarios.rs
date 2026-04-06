use crate::{handlers::usuarios::*, middleware::check::JWT};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/usuarios")
            .wrap(JWT)
            .route("/listar", web::post().to(listar_usuarios))
            .route("/crear", web::post().to(crear_usuario))
            .route("/editar", web::post().to(editar_usuario))
            .route("/eliminar", web::post().to(eliminar_usuario))
            .route("/reset_pass", web::post().to(reset_pass_usuario)),
    );
}
