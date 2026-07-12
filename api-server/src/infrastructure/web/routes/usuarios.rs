use crate::infrastructure::web::{handlers::usuarios::*, middleware::check::JWT};
use actix_web::web;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/usuarios")
            .wrap(JWT)
            .route("/listar", web::get().to(listar_usuarios))
            .route("/crear", web::post().to(crear_usuario))
            .route("/editar", web::put().to(editar_usuario))
            .route("/eliminar/{id}", web::delete().to(eliminar_usuario))
            .route("/reset_pass", web::post().to(reset_pass_usuario)),
    );
}
