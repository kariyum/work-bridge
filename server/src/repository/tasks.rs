use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(sqlx::FromRow, Serialize)]
pub struct RawTask {
    id: i32,
    project_id: i32,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    assignee: String,
    budget: f32,
    created_at: DateTime<Utc>,
}

pub async fn read_tasks_by_project_id(project_id: i32, conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawTask>, sqlx::Error> {
    sqlx::query_as::<_, RawTask>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(conn).await
}

struct CreateTask {
    project_id: i32,
    title: String,
    content: String,
    deadline: DateTime<Utc>,
    assignee: String,
    budget: f32,
}

pub async fn insert_task(create_task: CreateTask, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO tasks (project_id, title, content, deadline, assignee, budget)
                     VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(create_task.project_id)
        .bind(create_task.title)
        .bind(create_task.content)
        .bind(create_task.deadline)
        .bind(create_task.assignee)
        .bind(create_task.budget)
        .execute(conn)
        .await
        .map(|_| {})
}


mod test {
    use chrono::Utc;
    use sqlx::PgPool;
    use crate::repository::tasks::{insert_task, read_tasks_by_project_id, CreateTask};

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn insert_task_test(pg_pool: PgPool) {
        let create_task = CreateTask {
            project_id: 1,
            title: "Task 1".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            assignee: "Assignee1".to_string(),
            budget: 10.5,
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