use crate::handlers::login::*;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").route("/", web::post().to(login)));
}
