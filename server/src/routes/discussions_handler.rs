use crate::repository::discussions::get_user_discussions;
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_discussions(claims: Claims, pgpool: web::Data<PgPool>) -> impl Responder {
    let raw_discussions = get_user_discussions(&claims.sub, pgpool.as_ref())
        .await
        .expect(&format!("Failed to fetch user discussions {}", &claims.sub));

    HttpResponse::Ok().json(raw_discussions)
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("discussions").route("", web::get().to(get_discussions))
}
