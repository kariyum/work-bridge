use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Serialize)]
pub struct RawComment {
    pub id: i32,
    pub created_by: String,
    pub parent_comment_id: Option<i32>,
    pub post_id: i32,
    pub comment: String,
    pub ups: Vec<String>,
    pub downs: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn read_comments_by_post_id(post_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawComment>, sqlx::Error> {
    sqlx::query_as::<_, RawComment>("SELECT * FROM comments WHERE post_id = $1")
        .bind(&post_id)
        .fetch_all(conn)
        .await
}

pub struct CreateComment {
    pub post_id: i32,
    pub parent_comment_id: Option<i32>,
    pub comment: String,
    pub created_by: String,
}

pub async fn insert_comment(create_comment: CreateComment, conn: impl Executor<'_, Database=Postgres>) -> Result<RawComment, sqlx::Error> {
    sqlx::query_as::<_, RawComment>("INSERT INTO comments (post_id, parent_comment_id, comment, created_by) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(create_comment.post_id)
        .bind(create_comment.parent_comment_id)
        .bind(create_comment.comment)
        .bind(create_comment.created_by)
        .fetch_one(conn)
        .await
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use crate::repository::comments::{insert_comment, read_comments_by_post_id, CreateComment};

    #[sqlx::test(fixtures(path = "./fixtures", scripts("comments.sql")))]
    async fn read_comment_by_post_id_test(conn: PgPool) {
        let post_id = 1;
        let comments = read_comments_by_post_id(post_id, &conn)
            .await
            .expect(format!("Failed to fetch comments for post_id {}", post_id).as_str());

        assert!(!comments.is_empty());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("comments.sql")))]
    async fn insert_comment_test(conn: PgPool) {
        let create_comment = CreateComment {
            post_id: 1,
            parent_comment_id: None,
            comment: String::from("A comment body"),
            created_by: String::from("user@gmail.com")
        };

        let inserted_comment = insert_comment(create_comment, &conn)
            .await
            .expect("Failed to insert a comment");

        assert_eq!(inserted_comment.comment, String::from("A comment body"))
    }

}