use actix_web::web;

use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::get().to(handlers::post_handler::index))
            .route("", web::post().to(handlers::post_handler::create)),
    );
}
