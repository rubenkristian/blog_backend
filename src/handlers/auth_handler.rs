use actix_web::HttpResponse;

pub async fn signin() -> HttpResponse {
    HttpResponse::Ok().json("Auth")
}

pub async fn signout() -> HttpResponse {
    HttpResponse::Ok().json("Auth")
}
