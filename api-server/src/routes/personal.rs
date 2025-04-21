use crate::{handlers::personal::*, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal")
            .wrap(JWT)
            .route("/buscar", web::post().to(buscar_por_nombre))
            .route("/vinculos_por_dni", web::post().to(vinculos_por_dni))
            .route("/por_dni", web::post().to(perfil_por_dni))
            .route("/editar_por_dni", web::post().to(editar_perfil))
            .route(
                "/renuncia_por_vinculo",
                web::post().to(renuncia_por_vinculo),
            )
            .route(
                "/agregar_infobancaria",
                web::post().to(agregar_infobancaria),
            )
            .route(
                "/editar_infobancaria",
                web::post().to(editar_datos_bancarios),
            )
            .route("/agregar_gradoa", web::post().to(agregar_gradoacademico))
            .route("/editar_gradoa", web::post().to(editar_gradoacademico))
            .route("/banco_por_dni", web::post().to(banco_por_dni))
            .route("/grado_por_dni", web::post().to(grado_por_dni)),
    );
}
