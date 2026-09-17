pub mod handlers;
pub mod models;
pub mod repo;
pub mod service;

use crate::common::middleware::JWT;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .route("/", web::post().to(handlers::login))
            .route("/register", web::post().to(handlers::register)),
    )
    .service(
        web::scope("/usuarios")
            .wrap(JWT)
            .route("/listar", web::get().to(handlers::listar_usuarios))
            .route("/crear", web::post().to(handlers::crear_usuario))
            .route("/editar", web::put().to(handlers::editar_usuario))
            .route("/eliminar/{id}", web::delete().to(handlers::eliminar_usuario)),
    );
}
