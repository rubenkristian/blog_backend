pub mod menu;
pub mod post;
pub mod user;

use actix_web::web;

use crate::routes::api::admin;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .configure(admin::menu::init)
            .configure(admin::post::init)
            .configure(admin::user::init),
    );
}
