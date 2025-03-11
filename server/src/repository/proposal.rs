use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(Serialize, sqlx::Type, Debug)]
#[sqlx(type_name = "proposal_status", rename_all = "lowercase")]
enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct RawProposal {
    id: i32,
    user_id: String,
    task_id: i32,
    status: ProposalStatus,
    budget: Option<BigDecimal>,
    content: Option<String>,
    created_at: DateTime<Utc>,
}

pub async fn read_proposals(
    task_id: i32,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<RawProposal>, sqlx::Error> {
    sqlx::query_as!(RawProposal, "SELECT id, user_id, task_id, status as \"status: ProposalStatus\", budget, content, created_at FROM proposals")
        .fetch_all(conn)
        .await
}


use sqlx::FromRow;
use sqlx::Type;
use sqlx::types::BigDecimal;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "mood", rename_all = "lowercase")]
enum Mood {
    Happy,
    Sad,
    Neutral,
}

// Define the struct for the table
#[derive(Debug, FromRow)]
struct ExampleTable {
    id: i32,
    name: String,
    current_mood: Mood,
}

mod test {
    use sqlx::PgPool;
    use crate::repository::proposal::{read_proposals, ExampleTable};
    use crate::repository::proposal::Mood;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("users.sql", "tasks.sql", "proposals.sql")))]
    async fn insert_task_test(pg_pool: PgPool) {
        let x = read_proposals(1, &pg_pool)
            .await
            .unwrap();

        println!("Result is = {:?}", x);
    }

}