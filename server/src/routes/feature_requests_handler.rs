use crate::repository::feature_requests::{
    insert_feature_request, read_all_feature_request, CreateFeatureRequest, RawFeatureRequest,
};
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Deserialize)]
struct FeatureRequestPost {
    title: String,
    description: String,
}

async fn create_feature_request_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    Json(payload): Json<FeatureRequestPost>,
    claims: Claims,
) -> impl Responder {
    let create_feature_request = CreateFeatureRequest {
        title: payload.title,
        description: payload.description,
        created_by: claims.sub,
    };

    let insertion_result = insert_feature_request(create_feature_request, pg_pool.as_ref()).await;

    match insertion_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

impl From<RawFeatureRequest> for FeatureRequestGet {
    fn from(raw_feature_request: RawFeatureRequest) -> Self {
        FeatureRequestGet {
            id: raw_feature_request.id,
            title: raw_feature_request.title,
            created_by: raw_feature_request.created_by,
            description: raw_feature_request.description,
            ups: raw_feature_request.ups,
            downs: raw_feature_request.downs,
            created_at: raw_feature_request.created_at,
        }
    }
}

#[derive(Serialize)]
struct FeatureRequestGet {
    pub id: i32,
    pub title: String,
    pub created_by: String,
    pub description: String,
    pub ups: Vec<String>,
    pub downs: Vec<String>,
    pub created_at: DateTime<Utc>,
}

async fn get_all_feature_requests_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    _: Claims,
) -> impl Responder {
    let feature_requests: Result<Vec<FeatureRequestGet>, Error> =
        read_all_feature_request(pg_pool.as_ref())
            .await
            .into_iter()
            .map(From::from)
            .collect();

    match feature_requests {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("feature_request")
        .route("", web::post().to(create_feature_request_handler))
        .route("", web::get().to(get_all_feature_requests_handler))
}
