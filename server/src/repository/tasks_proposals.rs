use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};
use crate::repository::proposal::ProposalStatus;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct RawTaskProposal {
    pub id: i32,
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub deadline: DateTime<Utc>,
    pub assignee_id: String,
    pub budget: f32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub skills: Vec<String>,
    pub proposal_status: Option<ProposalStatus>,
    pub proposal_id: Option<i32>,
}

pub async fn read_tasks_with_submission_by_project_id(
    project_id: i32,
    user_id: String,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<RawTaskProposal>, sqlx::Error> {
    sqlx::query_as!(RawTaskProposal,
        r#"SELECT
            tasks.id, tasks.project_id, tasks.title, tasks.content, tasks.deadline,
            tasks.assignee_id, tasks.budget, tasks.status, tasks.created_at, tasks.skills,
            proposals.status as "proposal_status: Option<ProposalStatus>",
            proposals.id as "proposal_id: Option<i32>"
        FROM
            tasks LEFT JOIN proposals ON tasks.id = proposals.task_id AND proposals.user_id = $2
        WHERE tasks.project_id = $1;"#,
        project_id,
        user_id
    )
        .fetch_all(conn).await
}

#[cfg(test)]
mod test {
    use crate::repository::tasks::{insert_task, CreateTask};
    use crate::repository::tasks_proposals::read_tasks_with_submission_by_project_id;
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
            status: "todo".to_string(),
            skills: vec!["Skill1".to_string()],
        };
        let result = insert_task(create_task, &pg_pool).await;
        let result2 =
            read_tasks_with_submission_by_project_id(1, String::from("user_id"), &pg_pool).await;
        println!("{:?}", result2);

        assert!(result.is_ok());
        assert!(result2.is_ok());
    }
}
