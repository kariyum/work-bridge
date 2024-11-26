use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Executor, Postgres};
use crate::tasks::repo::TaskCreate;

#[derive(sqlx::FromRow)]
struct ProjectRaw {
    id: i32,
    user_id: String,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
    created_at: DateTime<Utc>,
}

async fn get_project_by_id(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<ProjectRaw>, sqlx::Error> {
    let project = sqlx::query_as::<_, ProjectRaw>("select * from projects where id = $1")
        .bind(project_id)
        .fetch_optional(conn)
        .await;
    project
}

async fn get_projects(conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<ProjectRaw>, sqlx::Error> {
    let projects = sqlx::query_as::<_, ProjectRaw>("select * from projects")
        .fetch_all(conn)
        .await;
    projects
}

#[derive(Deserialize)]
pub struct ProjectCreate {
    user_id: String,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    budget: f32,
    currency_code: String,
    tasks: Vec<TaskCreate>
}

async fn insert_project(create_project: ProjectCreate, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
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
        let project_details = ProjectCreate {
            user_id: "user@gmail.com".to_string(),
            title: "title".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            budget: 10.5,
            currency_code: "TD".to_string(),
            tasks: vec!()
        };

        insert_project(project_details, &conn)
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