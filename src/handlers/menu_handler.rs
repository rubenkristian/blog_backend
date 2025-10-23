use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json("User")
}

pub async fn create() -> HttpResponse {
    HttpResponse::Ok().json("Create User")
}
