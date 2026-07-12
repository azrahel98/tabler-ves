use crate::infrastructure::web::{handlers::fileserver::*, middleware::check::JWT};
use actix_web::web::{self};
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fileserver")
            .route("/{hash}", web::get().to(ver_archivo))
            .service(
                web::scope("")
                    .wrap(JWT)
                    .route("/upload", web::post().to(upload_file))
                    .route("/upload_batch", web::post().to(upload_batch))
                    .route("/registrar_url", web::post().to(registrar_url))
                    .route("/archivos_por_dni/{dni}", web::get().to(listar_archivos_dni))
                    .route("/eliminar_archivo/{id}", web::delete().to(eliminar_archivo))
                    .route("/asignar_documento", web::post().to(asignar_documento))
                    .route("/documentos/{dni}", web::get().to(documentos_por_dni))
                    .route("/renombrar_archivo", web::put().to(renombrar_archivo)),
            ),
    );
}
