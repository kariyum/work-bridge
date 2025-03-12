use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use sqlx::{Executor, Postgres};

#[derive(Serialize, sqlx::Type, Debug)]
#[sqlx(type_name = "proposal_status", rename_all = "lowercase")]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

#[derive(Serialize, Debug)]
pub struct RawProposal {
    id: i32,
    user_id: String,
    task_id: i32,
    status: ProposalStatus,
    budget: Option<BigDecimal>,
    content: Option<String>,
    created_at: DateTime<Utc>,
}

pub struct CreateProposal {
    pub user_id: String,
    pub task_id: i32,
    pub status: ProposalStatus,
    pub budget: Option<BigDecimal>,
    pub content: Option<String>,
}

pub async fn read_proposals(
    task_id: i32,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<RawProposal>, sqlx::Error> {
    sqlx::query_as!(RawProposal, "SELECT id, user_id, task_id, status as \"status: ProposalStatus\", budget, content, created_at FROM proposals WHERE task_id = $1", task_id)
        .fetch_all(conn)
        .await
}

pub async fn insert_proposal(
    insert_proposal: CreateProposal,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO proposals (user_id, task_id, status, budget, content) VALUES ($1, $2, $3, $4, $5)",
        insert_proposal.user_id,
        insert_proposal.task_id,
        insert_proposal.status as ProposalStatus,
        insert_proposal.budget,
        insert_proposal.content
    ).execute(conn).await?;

    Ok(())
}

mod test {
    use crate::repository::proposal::read_proposals;
    use sqlx::PgPool;

    #[sqlx::test(fixtures(
        path = "./fixtures",
        scripts("users.sql", "tasks.sql", "proposals.sql")
    ))]
    async fn read_proposal_test(pg_pool: PgPool) {
        let x = read_proposals(1, &pg_pool).await.unwrap();

        println!("Result is = {:?}", x);
    }
}
