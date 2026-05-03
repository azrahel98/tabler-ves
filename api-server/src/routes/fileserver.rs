use crate::{handlers::fileserver::*, middleware::check::JWT};
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
                    .route("/listar_archivos_dni", web::post().to(listar_archivos_dni))
                    .route("/eliminar_archivo", web::post().to(eliminar_archivo))
                    .route("/asignar_documento", web::post().to(asignar_documento))
                    .route("/documentos_por_dni", web::post().to(documentos_por_dni)),
            ),
    );
}
