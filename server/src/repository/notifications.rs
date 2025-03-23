use actix::Message;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use sqlx::{Executor, Postgres};

#[derive(Serialize, sqlx::Type, Debug, Clone, Message)]
#[rtype(result = "()")]
#[serde(rename_all(serialize = "snake_case"))]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    Proposal, // renamed to ProposalStatusChange
    NewProposal,
    Message,
}

#[derive(Serialize, Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct RawNotification {
    pub id: i32,
    pub user_id: String,
    pub content: Value,
    pub notification_type: NotificationType,
    pub created_at: DateTime<Utc>,
}

pub async fn read_notifications(
    user_id: String,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<RawNotification>, sqlx::Error> {
    sqlx::query_as!(
        RawNotification,
        "SELECT id, user_id, content, type as \"notification_type: NotificationType\", created_at FROM notifications WHERE user_id = $1",
        user_id
    )
        .fetch_all(conn)
        .await
}

#[derive(Serialize, Message, Clone)]
#[rtype(result = "()")]
pub struct CreateNotification {
    pub user_id: String,
    pub content: Value,
    pub notification_type: NotificationType
}

pub async fn insert_notification(
    create_notification: CreateNotification,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<RawNotification, sqlx::Error> {
    sqlx::query_as!(
        RawNotification,
        "INSERT INTO notifications (user_id, content, type) VALUES ($1, $2, $3) \
         RETURNING id, user_id, content, type as \"notification_type: NotificationType\", created_at",
        create_notification.user_id,
        create_notification.content,
        create_notification.notification_type as NotificationType
    )
    .fetch_one(conn)
    .await
}
#[cfg(test)]
mod test {
    use crate::repository::notifications::read_notifications;
    use sqlx::PgPool;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("users.sql", "notifications.sql")))]
    async fn read_notifications_test(pg_pool: PgPool) {
        let x = read_notifications(String::from("test@gmail.com"), &pg_pool)
            .await
            .unwrap();
        // let json_content = x.first().unwrap().content.clone();
        println!("Result is = {:?}", x);
    }
}
