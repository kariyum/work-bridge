use chrono::{DateTime, Utc};
use futures_util::{stream, StreamExt};
use serde::Serialize;
use sqlx::{Executor, PgPool, Postgres};

#[derive(sqlx::FromRow, Serialize)]
pub struct RawTask {
    id: i32,
    project_id: i32,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    assignee_id: String,
    budget: f32,
    status: String,
    created_at: DateTime<Utc>,
}

pub async fn read_tasks_by_project_id(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawTask>, sqlx::Error> {
    sqlx::query_as::<_, RawTask>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(conn).await
}

pub struct CreateTask {
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub deadline: DateTime<Utc>,
    pub assignee_id: String,
    pub budget: f32,
    pub status: String,
}

pub async fn insert_task(create_task: CreateTask, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO tasks (project_id, title, content, deadline, assignee_id, budget, status)
                     VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(create_task.project_id)
        .bind(create_task.title)
        .bind(create_task.content)
        .bind(create_task.deadline)
        .bind(create_task.assignee_id)
        .bind(create_task.budget)
        .bind(create_task.status)
        .execute(conn)
        .await
        .map(|_| {})
}

pub async fn insert_tasks_sequentially(tasks: Vec<CreateTask>, conn: &PgPool) -> Result<(), sqlx::Error> {
    for task in tasks {
        insert_task(task, conn).await.expect("Failed to insert task");
    }
    Ok(())
}

pub async fn insert_tasks_concurrently(tasks: Vec<CreateTask>, conn: &PgPool) -> Result<(), sqlx::Error> {
    let insertions = stream::iter(tasks)
        .map(|task| insert_task(task, conn))
        .buffer_unordered(3)
        .collect::<Vec<_>>()
        .await;

    for insertion in insertions {
        insertion.expect("Failed to insert task");
    }

    Ok(())
}

mod test {
    use crate::repository::tasks::{insert_task, read_tasks_by_project_id, CreateTask};
    use chrono::Utc;
    use sqlx::PgPool;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn insert_task_test(pg_pool: PgPool) {
        let create_task = CreateTask {
            project_id: 1,
            title: "Task 1".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            assignee_id: "Assignee1".to_string(),
            budget: 10.5,
            status: "todo".to_string()
        };
        let result = insert_task(create_task, &pg_pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("tasks.sql")))]
    async fn read_task_test(pg_pool: PgPool) {
        let project_id = 1;
        let result = read_tasks_by_project_id(project_id, &pg_pool).await;
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }
}