use actix_web::web;

use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/login").route("", web::post().to(handlers::auth_handler::signin)));
}
