use crate::security::token::validate_jwt;
use actix_web::{
    get,
    web::{self},
    HttpRequest, HttpResponse, Responder,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
struct PostRow {
    id: i64,
    title: String,
    content: String,
    user_id: i64,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[get("posts")]
pub async fn get_posts(request: HttpRequest, pgpool: web::Data<PgPool>) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();
    println!("Cookie is {:?}", cookie);
    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let posts = sqlx::query_as::<_, PostRow>("SELECT * FROM Posts")
        .fetch_all(&mut *client)
        .await
        .expect("Failed to fetch Posts");

    HttpResponse::Ok().json(posts)
}
