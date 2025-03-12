use crate::repository::messages::read_messages;
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Data, Path};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

async fn get_messages(_: Claims, path: Path<i32>, pgpool: Data<PgPool>) -> impl Responder {
    let discussion_id = path.into_inner();
    let messages = read_messages(discussion_id, pgpool.as_ref())
        .await
        .expect(&format!(
            "Failed to fetch message for discussion {}",
            discussion_id
        ));

    HttpResponse::Ok().json(messages)
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("messages").route("{discussion_id}", web::get().to(get_messages))
}
