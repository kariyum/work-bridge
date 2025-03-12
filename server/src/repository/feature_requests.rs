use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Serialize)]
pub struct RawFeatureRequest {
    pub id: i32,
    pub title: String,
    pub created_by: String,
    pub description: String,
    pub ups: Vec<String>,
    pub downs: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn read_all_feature_request(conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawFeatureRequest>, sqlx::Error> {
    sqlx::query_as::<_, RawFeatureRequest>("SELECT * FROM feature_requests")
        .fetch_all(conn)
        .await
}

pub struct CreateFeatureRequest {
    pub title: String,
    pub created_by: String,
    pub description: String,
}

pub async fn insert_feature_request(create_feature_request: CreateFeatureRequest, conn: impl Executor<'_, Database=Postgres>) -> Result<RawFeatureRequest, sqlx::Error> {
    sqlx::query_as::<_, RawFeatureRequest>("INSERT INTO feature_requests (title, created_by, description) VALUES ($1, $2, $3) RETURNING *")
        .bind(create_feature_request.title)
        .bind(create_feature_request.created_by)
        .bind(create_feature_request.description)
        .fetch_one(conn)
        .await
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use crate::repository::feature_requests::{insert_feature_request, read_all_feature_request, CreateFeatureRequest};

    #[sqlx::test(fixtures(path = "./fixtures", scripts("feature_requests.sql")))]
    async fn read_all_feature_requests_test(conn: PgPool) {
        let feature_requests = read_all_feature_request(&conn)
            .await
            .expect("Failed to fetch all feature requests");

        assert!(!feature_requests.is_empty());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("feature_requests.sql")))]
    async fn insert_feature_request_test(conn: PgPool) {
        let create_feature_request = CreateFeatureRequest {
            title: String::from("Feature request title"),
            created_by: String::from("user@gmail.com"),
            description: String::from("Feature description"),
        };

        let insertion_result = insert_feature_request(create_feature_request, &conn)
            .await
            .expect("Failed to insert feature request");

        assert_eq!(insertion_result.title, "Feature request title");
    }

}