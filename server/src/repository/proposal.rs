use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct RawProposal {
    id: i32,
    user_id: String,
    task_id: i32,
    status: i32,
    budget: Option<f32>,
    content: Option<String>,
    created_at: DateTime<Utc>,
}

pub async fn read_proposals(
    task_id: i32,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<RawProposal>, sqlx::Error> {
    sqlx::query_as::<_, RawProposal>("SELECT * FROM proposals WHERE task_id = $1;")
        .bind(task_id)
        .fetch_all(conn)
        .await
}
