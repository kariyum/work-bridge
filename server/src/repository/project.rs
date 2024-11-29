use crate::tasks::repo::TaskCreate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow)]
pub struct ProjectRaw {
    pub id: i32,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub deadline: DateTime<Utc>,
    pub budget: f32,
    pub currency_code: String,
    pub created_at: DateTime<Utc>,
}

pub async fn get_project_by_id(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<ProjectRaw>, sqlx::Error> {
    let project = sqlx::query_as::<_, ProjectRaw>("select * from projects where id = $1")
        .bind(project_id)
        .fetch_optional(conn)
        .await;
    project
}

pub async fn get_projects(conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<ProjectRaw>, sqlx::Error> {
    let projects = sqlx::query_as::<_, ProjectRaw>("select * from projects")
        .fetch_all(conn)
        .await;
    projects
}

#[derive(Deserialize)]
pub struct ProjectInsert {
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub deadline: DateTime<Utc>,
    pub budget: f32,
    pub currency_code: String,
}

pub async fn insert_project(create_project: ProjectInsert, conn: impl Executor<'_, Database=Postgres>) -> Result<ProjectRaw, sqlx::Error> {
    sqlx::query_as::<_, ProjectRaw>("
        INSERT INTO projects (user_id, title, content, deadline, budget, currency_code)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        ")
        .bind(create_project.user_id)
        .bind(create_project.title)
        .bind(create_project.content)
        .bind(create_project.deadline)
        .bind(create_project.budget)
        .bind(create_project.currency_code)
        .fetch_one(conn)
        .await

}

pub async fn delete_project(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(&project_id)
        .execute(conn)
        .await
        .expect(format!("Failed to delete project from the database {project_id}").as_str());

    Ok(())
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
        let project_details = ProjectInsert {
            user_id: "user@gmail.com".to_string(),
            title: "title".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            budget: 10.5,
            currency_code: "TD".to_string()
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

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn delete_project_test(conn: PgPool) {
        let projects = delete_project(1, &conn)
            .await;
        assert!(projects.is_ok());

    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("tasks.sql")))]
    async fn delete_project_with_tasks_test(conn: PgPool) {
        let projects = delete_project(1, &conn)
            .await;
        assert!(projects.is_ok());
    }
}