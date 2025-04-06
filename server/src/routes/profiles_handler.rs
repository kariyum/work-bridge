use crate::repository::profiles::{read_profile, upsert_profile, CreateProfile};
use crate::repository::user::{update_user, UserInfoUpdate};
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
struct ProfilePost {
    first_name: String,
    last_name: String,
    skills: Vec<String>,
    github_link: Option<String>,
    linkedin_link: Option<String>,
    portfolio_link: Option<String>,
    bio: Option<String>,
}

async fn create_profile_handler(
    pg_pool: web::Data<Pool<Postgres>>,
    Json(payload): Json<ProfilePost>,
    claims: Claims,
) -> impl Responder {
    let create_profile = CreateProfile {
        user_id: claims.sub.clone(),
        skills: payload.skills,
        bio: payload.bio,
        github_link: payload.github_link,
        linkedin_link: payload.linkedin_link,
        portfolio_link: payload.portfolio_link,
    };

    let insertion_result = upsert_profile(create_profile, pg_pool.as_ref()).await;
    let user_info_update = UserInfoUpdate {
        first_name: payload.first_name,
        last_name: payload.last_name,
    };

    if insertion_result.is_ok() {
        update_user(claims.sub, user_info_update, pg_pool.as_ref())
            .await
            .expect("Failed to update user")
    };

    match insertion_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[derive(Deserialize)]
pub struct OptionalPath {
    pub user_id: Option<String>,
}
async fn get_profile(user_id: String, pg_pool: &Pool<Postgres>) -> impl Responder {
    let profile = read_profile(user_id, pg_pool)
        .await
        .expect("Failed to read profile");

    match profile {
        Some(profile) => HttpResponse::Ok().json(profile),
        None => HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Resource not found for this user"),
    }
}
async fn get_profile_path(
    _: Claims,
    path: Path<String>,
    pg_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let user_id = path.into_inner();
    get_profile(user_id, pg_pool.as_ref()).await
}

async fn get_profile_claims(claims: Claims, pg_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    get_profile(claims.sub, pg_pool.as_ref()).await
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("profile")
        .route("", web::post().to(create_profile_handler))
        .route("", web::put().to(create_profile_handler))
        .route("", web::get().to(get_profile_claims))
        .route("{id}", web::get().to(get_profile_path))
}
