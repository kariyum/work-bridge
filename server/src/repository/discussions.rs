use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(Deserialize, sqlx::FromRow, Serialize)]
pub struct Discussion {
    pub id: i32,
    pub user_ids: Vec<String>,
    pub created_by: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_user_discussions(
    user_id: &String,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<Discussion>, sqlx::Error> {
    // let query = ;
    sqlx::query_as!(
        Discussion,
        "SELECT id, user_ids, created_by, title, created_at FROM discussions WHERE $1 = ANY(user_ids)",
        user_id
    )
        .fetch_all(conn)
        .await
}
