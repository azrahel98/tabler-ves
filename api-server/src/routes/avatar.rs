use crate::{handlers::avatar::upload_avatar, middleware::check::JWT};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/avatar")
            .wrap(JWT)
            .route("/upload/{id}", web::post().to(upload_avatar)),
    );
}
