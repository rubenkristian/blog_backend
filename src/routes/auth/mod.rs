pub mod login;
pub mod logout;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/"));
}
