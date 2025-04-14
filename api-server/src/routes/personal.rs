use crate::{
    handlers::personal::{
        banco_por_dni, buscar_por_nombre, grado_por_dni, perfil_por_dni, renuncia_por_vinculo,
        vinculos_por_dni,
    },
    middleware::check::JWT,
};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal")
            .wrap(JWT)
            .route("/buscar", web::post().to(buscar_por_nombre))
            .route("/vinculos_por_dni", web::post().to(vinculos_por_dni))
            .route("/por_dni", web::post().to(perfil_por_dni))
            .route(
                "/renuncia_por_vinculo",
                web::post().to(renuncia_por_vinculo),
            )
            .route("/banco_por_dni", web::post().to(banco_por_dni))
            .route("/grado_por_dni", web::post().to(grado_por_dni)),
    );
}
