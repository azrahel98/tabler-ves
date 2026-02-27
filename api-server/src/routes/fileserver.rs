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
                    .route("/listar_archivos_dni", web::post().to(listar_archivos_dni))
                    .route("/eliminar_archivo", web::post().to(eliminar_archivo)),
            ),
    );
}
