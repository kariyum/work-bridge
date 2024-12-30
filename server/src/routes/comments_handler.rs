use crate::repository::comments::{
    insert_comment, read_comments_by_post_id, CreateComment, RawComment,
};
use crate::repository::feature_requests::{
    insert_feature_request, read_all_feature_request, CreateFeatureRequest, RawFeatureRequest,
};
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct CommentPost {
    post_id: i32,
    parent_comment_id: Option<i32>,
    comment: String,
}

async fn create_feature_request_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    Json(payload): Json<CommentPost>,
    claims: Claims,
) -> impl Responder {
    let create_comment = CreateComment {
        post_id: payload.post_id,
        parent_comment_id: payload.parent_comment_id,
        comment: payload.comment,
        created_by: claims.sub,
    };

    let insertion_result = insert_comment(create_comment, pg_pool.as_ref()).await;

    match insertion_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[derive(Serialize)]
struct CommentGet {
    id: i32,
    created_by: String,
    parent_comment_id: Option<i32>,
    post_id: i32,
    comment: String,
    ups: Vec<String>,
    downs: Vec<String>,
    created_at: DateTime<Utc>,
}

impl From<RawComment> for CommentGet {
    fn from(value: RawComment) -> Self {
        CommentGet {
            id: value.id,
            created_by: value.created_by,
            parent_comment_id: value.parent_comment_id,
            post_id: value.post_id,
            comment: value.comment,
            ups: value.ups,
            downs: value.downs,
            created_at: value.created_at,
        }
    }
}

async fn get_comments_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    post_id: Path<i32>,
    _: Claims,
) -> impl Responder {
    let feature_requests: Result<Vec<CommentGet>, Error> =
        read_comments_by_post_id(post_id.into_inner(), pg_pool.as_ref())
            .await
            .map(|comments| comments.into_iter().map(CommentGet::from).collect());

    match feature_requests {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("comments")
        .route("", web::post().to(create_feature_request_handler))
        .route("/{id}", web::get().to(get_comments_handler))
}
