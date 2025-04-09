use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Path;
use sqlx::PgPool;
use crate::repository::discussions::{get_discussion_id, get_user_discussions};

pub async fn get_discussions(claims: Claims, pgpool: web::Data<PgPool>) -> impl Responder {
    let raw_discussions = get_user_discussions(&claims.sub, pgpool.as_ref())
        .await
        .expect(&format!("Failed to fetch user discussions {}", &claims.sub));

    HttpResponse::Ok().json(raw_discussions)
}

pub async fn get_discussion_id_by_user_ids(claims: Claims, path: Path<String>, pgpool: web::Data<PgPool>) -> impl Responder {
    let user_ids = path.into_inner().split(",")
        .chain(std::iter::once(claims.sub.as_str()))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let discussion_id = get_discussion_id(user_ids, pgpool.as_ref())
        .await
        .expect("Failed to fetch user discussion id");

    if let Some(discussion_id) = discussion_id {
        HttpResponse::Ok().json(discussion_id)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("discussions")
        .route("", web::get().to(get_discussions))
        .route("{user_ids}", web::get().to(get_discussion_id_by_user_ids))
}
