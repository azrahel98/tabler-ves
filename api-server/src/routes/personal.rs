use crate::{handlers::personal::*, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal")
            .wrap(JWT)
            .route("/agregar_gradoa", web::post().to(agregar_gradoacademico))
            .route(
                "/agregar_infobancaria",
                web::post().to(agregar_infobancaria),
            )
            .route("/agregar_sindicato", web::post().to(agregar_sindicato))
            .route("/banco_por_dni", web::post().to(banco_por_dni))
            .route("/buscar", web::post().to(buscar_por_nombre))
            .route("/editar_gradoa", web::post().to(editar_gradoacademico))
            .route(
                "/editar_infobancaria",
                web::post().to(editar_datos_bancarios),
            )
            .route("/editar_por_dni", web::post().to(editar_perfil))
            .route("/grado_por_dni", web::post().to(grado_por_dni))
            .route("/por_dni", web::post().to(perfil_por_dni))
            .route(
                "/renuncia_por_vinculo",
                web::post().to(renuncia_por_vinculo),
            )
            .route("/personas_legajo", web::post().to(personas_legajos))
            .route("/legajo_por_dni", web::post().to(reporte_legajo))
            .route(
                "/nuevo_evento_legajo",
                web::post().to(agregar_evento_legajo),
            )
            .route("/vinculos_por_dni", web::post().to(vinculos_por_dni))
            .route("/asistencia", web::post().to(report_asistencia)),
    );
}
