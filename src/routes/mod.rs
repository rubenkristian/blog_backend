pub mod api;
pub mod auth;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::init))
        .service(web::scope("/authentication").configure(auth::init));
}
