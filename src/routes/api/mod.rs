pub mod menu;
pub mod post;
pub mod user;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .configure(user::init)
            .configure(menu::init)
            .configure(post::init),
    );
}
