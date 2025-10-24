use actix_web::web;

use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(handlers::user_handler::index))
            .route("", web::post().to(handlers::user_handler::create)),
    );
}
