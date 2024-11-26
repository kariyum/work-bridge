use crate::{
    messaging::discussions,
    project::{self, repo::ProjectRow},
};
use actix_web::{
    delete, get, post,
    web::{self, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, PgPool, Postgres};
use crate::services::token::validate_jwt;

#[derive(Serialize, sqlx::FromRow)]
struct MessageRow {
    id: i32,
    from_user_id: String,
    discussion_id: i32,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct MessageCreate {
    pub discussion_id: i32,
    pub content: String, // add target user_ids for faster lookup when pushing messages to clients
}

pub async fn create_message(
    sender_id: String, // from claims.sub
    message_create: MessageCreate,
    mut client: PoolConnection<Postgres>,
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
    .execute(&mut *client)
    .await
    .expect("Failed to insert message into database");
}

#[get("/messages/{id}")]
pub async fn get_messages(
    request: HttpRequest,
    path: Path<i32>,
    pgpool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()).ok())
        .flatten();

    if cookie.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let mut client = pgpool
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");
    let discussion_id: i32 = path.into_inner();
    let messages = sqlx::query_as::<_, MessageRow>(
        "
        SELECT 
            * 
        FROM 
            messages 
        WHERE
            discussion_id = $1
        ORDER BY created_at ASC                   
        ",
    )
    .bind(&discussion_id)
    .fetch_all(&mut *client)
    .await
    .expect("Failed to retrieve messages from the database");

    HttpResponse::Ok().json(messages)
}
