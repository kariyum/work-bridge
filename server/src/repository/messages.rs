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
pub struct MessageRow {
    id: i32,
    from_user_id: String,
    discussion_id: i32,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn read_messages(
    discussion_id: i32,
    pgpool: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<MessageRow>, sqlx::Error> {
    sqlx::query_as!(
        MessageRow,
        "\
            SELECT \
                id, from_user_id, discussion_id, content, created_at \
            FROM \
                messages \
            WHERE \
                discussion_id = $1 \
            ORDER BY created_at ASC \
        ", // make sure the fetcher is in the discussion for privacy
        discussion_id
    )
    .fetch_all(pgpool)
    .await
}
