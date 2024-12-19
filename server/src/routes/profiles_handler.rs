use crate::repository::profiles::{insert_profile, CreateProfile};
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
struct ProfilePost {
    birthdate: DateTime<Utc>,
    skills: Vec<String>,
    phone: String,
}

async fn create_profile_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    Json(payload): Json<ProfilePost>,
    claims: Claims
) -> impl Responder {
   let create_profile = CreateProfile {
       user_id: claims.sub,
       birthdate: payload.birthdate,
       skills: payload.skills,
       phone: payload.phone
   };

    let insertion_result = insert_profile(create_profile, pg_pool.as_ref())
        .await;

    match insertion_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("profile")
        .route("", web::post().to(create_profile_handler))
}
