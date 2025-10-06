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
    sqlx::query_as!(
        Discussion,
        "SELECT id, user_ids, created_by, title, created_at FROM discussions WHERE $1 = ANY(user_ids)",
        user_id
    )
        .fetch_all(conn)
        .await
}

#[derive(Serialize)]
pub struct DiscussionId {
    pub id: i32,
}
pub async fn get_discussion_id(
    task_id: i32,
    proposal_id: i32,
    pgpool: impl Executor<'_, Database = Postgres>,
) -> Result<Option<DiscussionId>, sqlx::Error> {
    sqlx::query_as!(
        DiscussionId,
        r#"SELECT id FROM discussions WHERE task_id = $1 AND proposal_id = $2"#,
        task_id,
        proposal_id
    )
    .fetch_optional(pgpool)
    .await
}

pub struct CreateDiscussion {
    pub user_ids: Vec<String>,
    pub task_id: i32,
    pub proposal_id: i32,
    pub created_by: String,
}
pub async fn insert_discussion(
    create_discussion: CreateDiscussion,
    pgpool: impl Executor<'_, Database = Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO discussions (user_ids, task_id, proposal_id, created_by) VALUES ($1, $2, $3, $4)"#,
        &create_discussion.user_ids,
        &create_discussion.task_id,
        &create_discussion.proposal_id,
        &create_discussion.created_by
    )
    .execute(pgpool)
    .await
    .map(|_| {})
}

pub struct DiscussionIdWithMembers {
    pub id: i32,
    pub members: Vec<String>,
}
pub async fn get_discussion_id_with_members(
    user_id: String,
    task_id: i32,
    proposal_id: i32,
    pgpool: impl Executor<'_, Database = Postgres>,
) -> Result<Option<DiscussionIdWithMembers>, sqlx::Error> {
    sqlx::query_as!(DiscussionIdWithMembers, r#"SELECT id, user_ids as members FROM discussions WHERE task_id = $1 AND proposal_id = $2 AND $3 = ANY(user_ids)"#, task_id, proposal_id, user_id)
        .fetch_optional(pgpool)
        .await
}
