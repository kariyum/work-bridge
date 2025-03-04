use crate::repository::project::ProjectRaw;
use actix_web::{
    delete, get, post,
    web::{self, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::services::token::validate_jwt;

#[derive(Serialize, sqlx::FromRow)]
struct ProposalRow {
    id: i32,
    user_id: String,
    project_id: i32,
    status: i32,
    budget: Option<f32>,
    content: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct ProposalCreate {
    project_id: i32,
    status: Option<i32>,
    budget: Option<f32>,
    content: Option<String>,
}

#[post("proposals")]
pub async fn create_proposal(
    request: HttpRequest,
    proposal_create: Json<ProposalCreate>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if let Some(claims) = cookie {
        let mut client = pgpool
            .acquire()
            .await
            .expect("Failed to acquire a Postgres connection from the pool");

        let proposals = sqlx::query_as::<_, ProposalRow>(
            "INSERT INTO proposals (
                user_id,
                project_id,
                status,
                budget,
                content
            ) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
            .bind(&claims.sub)
            .bind(&proposal_create.project_id)
            .bind(0)
            .bind(&proposal_create.budget)
            .bind(&proposal_create.content)
            .fetch_optional(&mut *client)
            .await
            .expect("Failed to insert proposal into database");
        let project = sqlx::query_as::<_, ProjectRaw>("SELECT * FROM projects WHERE id = $1")
            .bind(&proposal_create.project_id)
            .fetch_optional(&mut *client)
            .await
            .expect("Failed to fetch project from database");
        if let Some(project) = project {
            let exists = sqlx::query("SELECT * FROM discussions where user_ids = $1")
                .bind(vec![&claims.sub, &project.user_id])
                .fetch_optional(&mut *client)
                .await
                .expect("Failed to fetch discussion from database")
                .is_some();
            if !exists {
                sqlx::query("INSERT INTO discussions (user_ids, created_by, created_at) VALUES ($1, $2, $3) RETURNING *")
                    .bind(vec![&claims.sub, &project.user_id])
                    .bind(&claims.sub)
                    .bind(chrono::Utc::now())
                    .execute(&mut *client)
                    .await
                    .expect("Failed to insert discussion into database");
            }
        }

        HttpResponse::Created().json(proposals)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/")]
pub async fn get_proposals(request: HttpRequest, pgpool: web::Data<PgPool>) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let proposals = sqlx::query_as::<_, ProposalRow>("SELECT * FROM proposals")
        .fetch_all(&mut *client)
        .await
        .expect("Failed to insert project into database");

    HttpResponse::Ok().json(proposals)
}

#[delete("proposals/{id}")]
pub async fn delete_proposal(
    request: HttpRequest,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let proposal_id = path.into_inner();
    let _ = sqlx::query("DELETE FROM proposals WHERE id = $1")
        .bind(&proposal_id)
        .execute(&mut *client)
        .await
        .expect(format!("Failed to delete project from the database {proposal_id}").as_str());

    HttpResponse::Ok().finish()
}

#[get("proposals/{id}")]
pub async fn get_proposal(
    request: HttpRequest,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let proposal_id = path.into_inner();
    let proposal = sqlx::query_as::<_, ProposalRow>("SELECT * FROM proposals WHERE id = $1")
        .bind(&proposal_id)
        .fetch_optional(&mut *client)
        .await
        .expect(format!("Failed to delete project from the database {proposal_id}").as_str());

    HttpResponse::Ok().json(proposal)
}
