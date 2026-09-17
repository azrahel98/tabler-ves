pub mod handlers;

use crate::common::middleware::JWT;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fileserver").service(
            web::scope("")
                .wrap(JWT)
                .route("/{hash}", web::get().to(handlers::ver_archivo))
                .route("/upload", web::post().to(handlers::upload_file))
                .route("/upload_batch", web::post().to(handlers::upload_batch))
                .route("/registrar_url", web::post().to(handlers::registrar_url))
                .route(
                    "/archivos_por_dni/{dni}",
                    web::get().to(handlers::listar_archivos_dni),
                )
                .route(
                    "/eliminar_archivo/{id}",
                    web::delete().to(handlers::eliminar_archivo),
                )
                .route(
                    "/asignar_documento",
                    web::post().to(handlers::asignar_documento),
                )
                .route(
                    "/documentos/{dni}",
                    web::get().to(handlers::documentos_por_dni),
                )
                .route(
                    "/renombrar_archivo",
                    web::put().to(handlers::renombrar_archivo),
                ),
        ),
    );
}
