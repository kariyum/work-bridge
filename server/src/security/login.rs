use actix_web::{
    cookie::{time::Duration, Cookie},
    options, post,
    web::Form, HttpResponse, Responder,
};

use serde::Deserialize;
use super::token::generate_cookie;

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
// async fn login(info: web::Json<LoginRequest>) -> impl Responder {
pub async fn login(Form(info): Form<LoginRequest>) -> impl Responder {
    let cookie = generate_cookie(info.email.as_str()).unwrap();
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