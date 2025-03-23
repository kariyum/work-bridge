use crate::repository::notifications::{insert_notification, CreateNotification, NotificationType};
use crate::repository::proposal::{insert_proposal, read_proposal, read_proposal_for_notification, read_proposals, update_proposal_status, CreateProposal, ProposalStatus};
use crate::repository::tasks::{read_task_creator_by_id, TaskCreator};
use crate::services::token::Claims;
use crate::websocket::lobby::Lobby;
use actix::Addr;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse, Responder};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
        content: proposal_create.content,
    };

    insert_proposal(create_proposal, pgpool.as_ref())
        .await
        .expect("Failed to insert proposal");

    let task_creator = read_task_creator_by_id(proposal_create.task_id, pgpool.as_ref())
        .await
        .expect("Failed to fetch task creator by id");

    if let Some(TaskCreator {
        user_id: project_creator,
    }) = task_creator
    {
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

#[derive(Deserialize)]
struct ProposalAction {
    action: ProposalActions,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ProposalActions {
    Accept,
    Decline,
    Cancel,
}

async fn update_proposal_status_handler(
    claims: Claims,
    lobby: web::Data<Addr<Lobby>>,
    path: Path<i32>,
    Json(ProposalAction { action }): Json<ProposalAction>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let proposal_id = path.into_inner();
    let target_status = match action {
        ProposalActions::Accept => ProposalStatus::Accepted,
        ProposalActions::Decline => ProposalStatus::Declined,
        ProposalActions::Cancel => ProposalStatus::Cancelled,
    };
    update_proposal_status(proposal_id, target_status.clone(), pgpool.as_ref())
        .await
        .expect("Failed to update Proposal status");
    let proposal = read_proposal_for_notification(proposal_id, pgpool.as_ref())
        .await
        .expect("Failed to read proposal");
    if let Some(proposal) = proposal {
        let create_notification = CreateNotification {
            user_id: proposal.user_id.clone(),
            content: json!({
                "proposal_id": proposal.id,
                "proposal_status": target_status,
                "task_id": proposal.task_id,
                "project_id": proposal.project_id,
                "trigger_user_id": claims.sub
            }),
            notification_type: NotificationType::Proposal,
        };
        let inserted_notification = insert_notification(create_notification.clone(), pgpool.as_ref())
            .await
            .expect("Failed to insert proposal notification");

        lobby.do_send(inserted_notification);
    }

    HttpResponse::Ok().finish()
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("proposals")
        .route("", web::post().to(create_proposal))
        .route("", web::get().to(get_proposals))
        .route("{id}", web::delete().to(delete_proposal))
        .route("{id}", web::get().to(get_proposal))
        .route(
            "{id}/status",
            web::patch().to(update_proposal_status_handler),
        )
}
