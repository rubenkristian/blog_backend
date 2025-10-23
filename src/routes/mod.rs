pub mod api;
pub mod auth;

use actix_web::{
    HttpResponse, Result,
    body::EitherBody,
    dev,
    http::{StatusCode, header},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger},
    web,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde_json::json;

use crate::auth::middleware::jwt_middleware;

fn add_error_handler_unauthenticated<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    // Create the JSON response body as BoxBody
    let json_body = HttpResponse::Unauthorized()
        .json(json!({
            "error": "Unauthorized",
            "message": "Authentication required"
        }))
        .into_body();

    // Set the Content-Type header
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // Map the body into EitherBody::right so it matches ErrorHandlerResponse
    let res = res.map_body(|_, _| EitherBody::right(json_body));

    Ok(ErrorHandlerResponse::Response(res))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(Logger::default())
            .wrap(HttpAuthentication::bearer(jwt_middleware))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::UNAUTHORIZED, add_error_handler_unauthenticated),
            )
            .configure(api::init),
    )
    .service(
        web::scope("/authentication")
            .wrap(Logger::default())
            .configure(auth::init),
    );
}
