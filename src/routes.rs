use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::auth::middleware::jwt_middleware;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").wrap(HttpAuthentication::bearer(jwt_middleware)));
}
