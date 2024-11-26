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
use crate::repository::user::{self, get_user_by_credentials};

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
    let maybe_user_row = get_user_by_credentials(form.email.clone(), form.password.clone(), pg_pool.as_ref())
        .await;
    match maybe_user_row {
        Ok(Some(user_row)) => {
            let cookie = generate_cookie(form.email.as_str(), user_row.role.as_str()).unwrap();
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
