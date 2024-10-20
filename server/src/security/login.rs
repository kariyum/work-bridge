use std::hash::{DefaultHasher, Hash, Hasher};

use actix_web::{
    cookie::{time::Duration, Cookie},
    options, post,
    web::{self, Form},
    HttpResponse, Responder,
};
use sqlx::{pool::maybe, Pool, Postgres};

use super::token::generate_cookie;
use serde::Deserialize;
use crate::repo::user::{self, get_user};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    Form(form): Form<LoginRequest>,
    pg_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let pg_client = pg_pool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");
    
    let mut hasher = DefaultHasher::new();
    form.password.hash(&mut hasher);
    let hashed_password = hasher.finish().to_string();
    let maybe_user_row = get_user(form.email.clone(), hashed_password.clone(), pg_client).await;
    match maybe_user_row {
        Some(user_row) if user_row.hashed_password == hashed_password => {
            let cookie = generate_cookie(form.email.as_str()).unwrap();
            let mut response = HttpResponse::Ok().finish();
            response.add_cookie(&cookie).unwrap();
            response
        }
        _ => HttpResponse::Unauthorized().finish()
    }
}

#[options("/login")]
pub async fn preflight() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .finish()
}
