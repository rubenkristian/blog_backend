use actix_web::{App, HttpServer, web};
use env_logger::Env;

use crate::db::init_db;
mod auth;
mod db;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = init_db().await;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
