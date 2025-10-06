use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(Deserialize)]
pub struct MessageCreate {
    pub discussion_id: i32,
    pub content: String, // add target user_ids for faster lookup when pushing messages to clients
}

pub async fn insert_message(
    sender_id: String, // from claims.sub
    message_create: MessageCreate,
    pgpool: impl Executor<'_, Database = Postgres>,
) {
    println!("Saving message to database {:?}", message_create.content);
    sqlx::query(
        "INSERT INTO messages (
                from_user_id,
                discussion_id,
                content
            ) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&sender_id)
    .bind(&message_create.discussion_id)
    .bind(&message_create.content)
    .execute(pgpool)
    .await
    .expect("Failed to insert message into database");
}

#[derive(Serialize, sqlx::FromRow)]
pub struct LightMessage {
    id: i32,
    from_user_id: String,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn read_messages_by_proposal_and_task(
    discussion_id: i32,
    pgpool: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<LightMessage>, sqlx::Error> {
    sqlx::query_as!(
        LightMessage,
        r#"SELECT id, from_user_id, content, created_at from messages where discussion_id = $1"#,
        discussion_id
    )
    .fetch_all(pgpool)
    .await
}
