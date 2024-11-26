use std::hash::{Hash, Hasher};

use actix_web::{
    post,
    web::{self},
    HttpRequest, HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use actix_web::get;
use serde::Deserialize;
use serde::Serialize;
use crate::services::token::validate_jwt;

#[derive(Deserialize, sqlx::FromRow, Serialize)]
struct Discussion {
    id: i32,
    user_ids: Vec<String>,
    created_by: String,
    title: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[post("/discussions")]
pub async fn post_discussions(
    request: HttpRequest,
    discussion_request: web::Json<Discussion>,
    pg_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();
    match cookie {
        Some(claims) => {
            let mut pg_client = pg_pool
                .acquire()
                .await
                .expect("Failed to acquire a Postgres connection from the pool");
            let payload = discussion_request.into_inner();
            let discussions: Option<Discussion> = sqlx::query_as::<_, Discussion>(
                "INSERT INTO discussions (user_ids, created_by, title, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
            )
            .bind(&payload.user_ids)
            .bind(&claims.sub)
            .bind(&payload.title)
            .bind(&payload.created_at)
            .fetch_optional(&mut *pg_client)
            .await
            .expect("Failed to fetch discussions");

            HttpResponse::Ok().json(discussions)
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/discussions")]
pub async fn get_discussions(
    request: HttpRequest,
    pg_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();
    match cookie {
        Some(claims) => {
            let mut pg_client = pg_pool
                .acquire()
                .await
                .expect("Failed to acquire a Postgres connection from the pool");
            let discussions: Vec<Discussion> = sqlx::query_as::<_, Discussion>(
                "SELECT * FROM discussions WHERE $1 = ANY(user_ids)",
            )
            .bind(&claims.sub)
            .fetch_all(&mut *pg_client)
            .await
            .expect("Failed to fetch discussions");

            HttpResponse::Ok().json(discussions)
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}
