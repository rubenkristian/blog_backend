use actix_web::web;

use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/menu")
            .route("", web::get().to(handlers::menu_handler::index))
            .route("", web::post().to(handlers::menu_handler::create)),
    );
}
