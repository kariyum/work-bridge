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
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}



#[post("/register")]
pub async fn register(
    Form(register_request): Form<RegisterRequest>,
    data: web::Data<Pool<Postgres>>,
) -> impl Responder {
    HttpResponse::Ok().body("Register")
}
