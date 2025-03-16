use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::scope;
use sqlx::PgPool;
use crate::repository::notifications::read_notifications;
use crate::services::token::Claims;

async fn get_notifications(
    claims: Claims,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let notifications = read_notifications(claims.sub, pgpool.as_ref())
        .await
        .expect("Failed to fetch notifications");

    HttpResponse::Ok().json(notifications)
}

pub fn routes() -> impl HttpServiceFactory {
    scope("notifications")
        .route("", web::get().to(get_notifications))
}