use crate::repository::discussions::get_discussion_id_with_members;
use crate::repository::messages::{read_messages_by_proposal_and_task, LightMessage};
use crate::services::token::Claims;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct TaskProposalDiscussion {
    discussion_id: i32,
    members: Vec<String>,
    messages: Vec<LightMessage>,
}
#[derive(Debug, Deserialize)]
pub struct MessagePath {
    task_id: i32,
    proposal_id: i32,
}
pub async fn get_messages(
    claims: Claims,
    path: Path<MessagePath>,
    pgpool: Data<PgPool>,
) -> impl Responder {
    let MessagePath {
        task_id,
        proposal_id,
    } = path.into_inner();
    let discussion =
        get_discussion_id_with_members(claims.sub, task_id, proposal_id, pgpool.as_ref())
            .await
            .expect("Failed to fetch discussion id");
    if let Some(discussion) = discussion {
        let messages = read_messages_by_proposal_and_task(discussion.id, pgpool.as_ref())
            .await
            .expect("Failed to fetch messages");
        let result = TaskProposalDiscussion {
            discussion_id: discussion.id,
            members: discussion.members,
            messages,
        };
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// pub fn routes() -> impl HttpServiceFactory {
//     web::scope("messages").route("{discussion_id}", web::get().to(get_messages))
// }
