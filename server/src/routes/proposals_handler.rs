use crate::repository::proposal::{insert_proposal, read_proposals, CreateProposal, ProposalStatus};
use crate::repository::tasks::{read_task_creator_by_id, TaskCreator};
use crate::services::token::Claims;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use bigdecimal::BigDecimal;
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
pub struct ProposalCreate {
    task_id: i32,
    budget: Option<BigDecimal>,
    content: Option<String>,
}

pub async fn create_proposal(
    claims: Claims,
    Json(proposal_create): Json<ProposalCreate>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {

    let create_proposal = CreateProposal {
        user_id: claims.sub.clone(),
        task_id: proposal_create.task_id,
        status: ProposalStatus::Pending,
        budget: proposal_create.budget,
        content: proposal_create.content
    };

    insert_proposal(create_proposal, pgpool.as_ref())
        .await
        .expect("Failed to insert proposal");

    let task_creator = read_task_creator_by_id(proposal_create.task_id, pgpool.as_ref())
        .await
        .expect("Failed to fetch task creator by id");

    if let Some(TaskCreator { user_id: project_creator }) = task_creator {
        let exists = sqlx::query("SELECT * FROM discussions where user_ids = $1")
            .bind(vec![&claims.sub, &project_creator])
            .fetch_optional(pgpool.as_ref())
            .await
            .expect("Failed to fetch discussion from database")
            .is_some();

        if !exists {
            sqlx::query("INSERT INTO discussions (user_ids, created_by, created_at) VALUES ($1, $2, $3) RETURNING *")
                .bind(vec![&claims.sub, &project_creator])
                .bind(&claims.sub)
                .bind(chrono::Utc::now())
                .execute(pgpool.as_ref())
                .await
                .expect("Failed to insert discussion into database");
        }
    }

    HttpResponse::Created()
}

pub async fn get_proposals(
    _: Claims,
    path: Path<(i32, i32)>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let (_, task_id) = path.into_inner();
    let proposals = read_proposals(task_id, pgpool.as_ref())
        .await
        .expect("Failed to read proposals");
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
