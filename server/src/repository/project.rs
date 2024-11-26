use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow)]
struct RawProject {
    id: i32,
    user_id: String,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
    created_at: DateTime<Utc>,
}

async fn get_project_by_id(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<RawProject>, sqlx::Error> {
    let project = sqlx::query_as::<_, RawProject>("select * from projects where id = $1")
        .bind(project_id)
        .fetch_optional(conn)
        .await;
    project
}

async fn get_projects(conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawProject>, sqlx::Error> {
    let projects = sqlx::query_as::<_, RawProject>("select * from projects")
        .fetch_all(conn)
        .await;
    projects
}

#[derive(Deserialize)]
pub struct CreateProject {
    user_id: String,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
}

async fn project_create(create_project: CreateProject, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("
        INSERT INTO projects (user_id, title, content, deadline, budget, currency_code)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        ")
        .bind(create_project.user_id)
        .bind(create_project.title)
        .bind(create_project.content)
        .bind(create_project.deadline)
        .bind(create_project.budget)
        .bind(create_project.currency_code)
        .execute(conn)
        .await
        .map(|_| ())
}

mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn get_project_by_id_test(conn: PgPool) {
        let project_id = 1;
        let project = get_project_by_id(1, &conn)
            .await
            .expect(format!("Failed to fetch project by id {}", project_id).as_str());

        assert!(project.is_some());
        assert_eq!(project.unwrap().id, project_id);
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn create_project_test(conn: PgPool) {
        let project_details = CreateProject {
            user_id: "user@gmail.com".to_string(),
            title: "title".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            budget: 10.5,
            currency_code: "TD".to_string(),
        };

        project_create(project_details, &conn)
            .await
            .expect("Failed to create project");
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn get_all_projects_test(conn: PgPool) {
        let projects = get_projects(&conn)
            .await
            .unwrap();
        assert_ne!(projects.len(), 0);

    }
}