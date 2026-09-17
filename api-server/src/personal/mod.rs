pub mod handlers;
pub mod models;
pub mod repo;
pub mod service;

use crate::common::middleware::JWT;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal").service(
            web::scope("")
                .wrap(JWT)
                .route("/avatar/{dni}", web::get().to(handlers::ver_avatar))
                .route(
                    "/agregar_gradoa",
                    web::post().to(handlers::upsert_gradoacademico),
                )
                .route(
                    "/agregar_infobancaria",
                    web::post().to(handlers::agregar_infobancaria),
                )
                .route(
                    "/agregar_sindicato",
                    web::post().to(handlers::agregar_sindicato),
                )
                .route("/banco/{dni}", web::get().to(handlers::banco_por_dni))
                .route("/buscar", web::get().to(handlers::buscar_por_nombre))
                .route(
                    "/editar_infobancaria",
                    web::put().to(handlers::editar_datos_bancarios),
                )
                .route("/editar_por_dni", web::put().to(handlers::editar_perfil))
                .route("/grado/{dni}", web::get().to(handlers::grado_por_dni))
                .route("/perfil/{dni}", web::get().to(handlers::perfil_por_dni))
                .route(
                    "/renuncia_por_vinculo",
                    web::post().to(handlers::renuncia_por_vinculo),
                )
                .route(
                    "/documento/{id}",
                    web::get().to(handlers::obtener_documento_por_id),
                )
                .route(
                    "/crear_documento",
                    web::post().to(handlers::crear_documento),
                )
                .route("/documento", web::post().to(handlers::crear_documento))
                .route(
                    "/editar_documento",
                    web::put().to(handlers::editar_documento),
                )
                .route(
                    "/eliminar_documento/{id}",
                    web::delete().to(handlers::eliminar_documento),
                )
                .route(
                    "/documento/{id}",
                    web::delete().to(handlers::eliminar_documento),
                )
                .route("/vinculos/{dni}", web::get().to(handlers::vinculos_por_dni))
                .route(
                    "/agregar_contacto",
                    web::post().to(handlers::contacto_emergencia_add),
                )
                .route("/buscar_vacantes", web::get().to(handlers::buscar_vacantes))
                .route(
                    "/buscar_por_plaza",
                    web::get().to(handlers::buscar_por_plaza),
                )
                .route("/contacto/{dni}", web::get().to(handlers::conctaco_por_dni))
                .route(
                    "/registrar_trabajador",
                    web::post().to(handlers::registrar_trabajador),
                )
                .route(
                    "/consultar_dni/{dni}",
                    web::get().to(handlers::consultar_dni_reniec),
                )
                .route(
                    "/eliminar_vinculo/{id}",
                    web::delete().to(handlers::eliminar_vinculo),
                )
                .route("/buscar_areas", web::get().to(handlers::buscar_areas))
                .route("/buscar_cargos", web::get().to(handlers::buscar_cargos))
                .route(
                    "/upsert_evento_vinculo",
                    web::put().to(handlers::upsert_evento_vinculo),
                )
                .route(
                    "/delete_evento_vinculo/{id}",
                    web::delete().to(handlers::delete_evento_vinculo),
                )
                .route(
                    "/eventos_vinculo/{vinculo_id}",
                    web::get().to(handlers::eventos_por_vinculo),
                )
                .route(
                    "/eliminar_contacto/{id}",
                    web::delete().to(handlers::eliminar_contacto),
                )
                .route(
                    "/eliminar_sindicato/{id}",
                    web::delete().to(handlers::eliminar_sindicato),
                )
                .route(
                    "/eliminar_gradoa/{id}",
                    web::delete().to(handlers::eliminar_gradoa),
                )
                .route(
                    "/activos_por_distrito",
                    web::get().to(handlers::activos_por_distrito),
                )
                .route(
                    "/cambio_area",
                    web::post().to(handlers::registrar_cambio_area),
                )
                .route("/calidad_datos", web::get().to(handlers::calidad_datos))
                .route("/avatar", web::post().to(handlers::subir_avatar)),
        ),
    );
}
