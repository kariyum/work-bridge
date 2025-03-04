use crate::repository::project::ProjectRaw;
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
struct ProposalRow {
    id: i32,
    user_id: String,
    task_id: i32,
    status: i32,
    budget: Option<f32>,
    content: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct ProposalCreate {
    task_id: i32,
    status: Option<i32>,
    budget: Option<f32>,
    content: Option<String>,
}

pub async fn create_proposal(
    claims: Claims,
    proposal_create: Json<ProposalCreate>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let proposals = sqlx::query_as::<_, ProposalRow>(
        "INSERT INTO proposals (
                user_id,
                task_id,
                status,
                budget,
                content
            ) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(&claims.sub)
    .bind(&proposal_create.task_id)
    .bind(0)
    .bind(&proposal_create.budget)
    .bind(&proposal_create.content)
    .fetch_optional(&mut *client)
    .await
    .expect("Failed to insert proposal into database");

    let project = sqlx::query_as::<_, ProjectRaw>("SELECT * FROM projects WHERE id = $1")
        .bind(&proposal_create.task_id)
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
}

pub async fn get_proposals(_: Claims, pgpool: web::Data<PgPool>) -> impl Responder {
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

pub async fn delete_proposal(
    _: Claims,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let task_id = path.into_inner();
    let _ = sqlx::query("DELETE FROM proposals WHERE id = $1")
        .bind(&task_id)
        .execute(&mut *client)
        .await
        .expect(format!("Failed to delete proposal from the database {task_id}").as_str());

    HttpResponse::Ok().finish()
}

pub async fn get_proposal(_: Claims, path: Path<i32>, pgpool: web::Data<PgPool>) -> impl Responder {
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

pub fn routes() -> impl HttpServiceFactory {
    web::scope("proposals")
        .route("", web::post().to(create_proposal))
        .route("", web::get().to(get_proposals))
        .route("{id}", web::delete().to(delete_proposal))
        .route("{id}", web::get().to(get_proposal))
}
