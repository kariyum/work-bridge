use crate::repository::user::{get_user_by_credentials, insert_user, RegisterRequest};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Form;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use crate::services::token::{generate_cookie, Claims};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login(
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

pub async fn register(
    Form(register_request): Form<RegisterRequest>,
    data: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let _ = insert_user(&register_request, data.as_ref())
        .await
        .expect("Failed to insert user");

    let cookie = generate_cookie(&register_request.email, &register_request.role)
        .expect("Failed to generate cookie");

    HttpResponse::Ok().cookie(cookie).finish()
}


async fn logout() -> impl Responder {
    let cookie = Cookie::build("Authorization", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();

    HttpResponse::NoContent().cookie(cookie).finish()
}

async fn whoami(claims: Claims) -> impl Responder {
    HttpResponse::Ok().json(claims)
}
pub fn routes() -> impl HttpServiceFactory {
    web::scope("auth")
        .route("login", web::post().to(login))
        .route("register", web::post().to(register))
        .route("logout", web::get().to(logout))
        .route("whoami", web::get().to(whoami))
}