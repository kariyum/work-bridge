use actix_web::{
    cookie::Cookie,
    get,
    http::header,
    middleware::Logger,
    options, post,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};

use serde::{Deserialize, Serialize};
use super::token::generate_jwt;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
// async fn login(info: web::Json<LoginRequest>) -> impl Responder {
pub async fn login(Form(info): Form<LoginRequest>) -> impl Responder {
    let cookie = Cookie::build("Authorization", generate_jwt(&info.email).unwrap())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    let mut request = HttpResponse::Ok().finish();
    request.add_cookie(&cookie).unwrap();
    request
}


#[options("/login")]
pub async fn preflight() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .finish()
}