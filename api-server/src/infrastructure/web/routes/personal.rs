use crate::infrastructure::web::{
    handlers::{grado::*, personal::*},
    middleware::check::JWT,
};
use actix_web::web::{self};
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personal")
            .route("/avatar/{dni}", web::get().to(ver_avatar))
            .service(
                web::scope("")
                    .wrap(JWT)
                    .route("/agregar_gradoa", web::post().to(upsert_gradoacademico))
                    .route("/agregar_infobancaria", web::post().to(agregar_infobancaria))
                    .route("/agregar_sindicato", web::post().to(agregar_sindicato))
                    .route("/banco/{dni}", web::get().to(banco_por_dni))
                    .route("/buscar", web::get().to(buscar_por_nombre))
                    .route("/editar_infobancaria", web::put().to(editar_datos_bancarios))
                    .route("/editar_por_dni", web::put().to(editar_perfil))
                    .route("/grado/{dni}", web::get().to(grado_por_dni))
                    .route("/perfil/{dni}", web::get().to(perfil_por_dni))
                    .route("/renuncia_por_vinculo", web::post().to(renuncia_por_vinculo))
                    .route("/documento/{id}", web::get().to(obtener_documento_por_id))
                    .route("/crear_documento", web::post().to(crear_documento))
                    .route("/documento", web::post().to(crear_documento))
                    .route("/editar_documento", web::put().to(editar_documento))
                    .route("/eliminar_documento/{id}", web::delete().to(eliminar_documento))
                    .route("/documento/{id}", web::delete().to(eliminar_documento))
                    .route("/vinculos/{dni}", web::get().to(vinculos_por_dni))
                    .route("/agregar_contacto", web::post().to(contacto_emergencia_add))
                    .route("/buscar_vacantes", web::get().to(buscar_vacantes))
                    .route("/buscar_por_plaza", web::get().to(buscar_por_plaza))
                    .route("/contacto/{dni}", web::get().to(conctaco_por_dni))
                    .route("/registrar_trabajador", web::post().to(registrar_trabajador))
                    .route("/consultar_dni/{dni}", web::get().to(consultar_dni_reniec))
                    .route("/eliminar_vinculo/{id}", web::delete().to(eliminar_vinculo))
                    .route("/buscar_areas", web::get().to(buscar_areas))
                    .route("/buscar_cargos", web::get().to(buscar_cargos))
                    .route("/upsert_evento_vinculo", web::put().to(upsert_evento_vinculo))
                    .route("/delete_evento_vinculo/{id}", web::delete().to(delete_evento_vinculo))
                    .route("/eliminar_contacto/{id}", web::delete().to(eliminar_contacto))
                    .route("/eliminar_sindicato/{id}", web::delete().to(eliminar_sindicato))
                    .route("/eliminar_gradoa/{id}", web::delete().to(eliminar_gradoa))
                    .route("/activos_por_distrito", web::get().to(activos_por_distrito))
                    .route("/cambio_area", web::post().to(registrar_cambio_area))
                    .route("/calidad_datos", web::get().to(calidad_datos))
                    .route("/avatar", web::post().to(subir_avatar)),
            ),
    );
}
