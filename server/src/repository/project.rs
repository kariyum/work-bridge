use chrono::{DateTime, Duration, Utc};
use fake::faker::chrono::pt_br::DateTimeBetween;
use fake::{
    faker::lorem::en::Sentence,
    rand::seq::IndexedRandom,
    Dummy, Fake, Faker,
};
use serde::Deserialize;
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

pub async fn get_project_by_id(
    project_id: i32,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Option<ProjectRaw>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRaw>("select * from projects where id = $1")
        .bind(project_id)
        .fetch_optional(conn)
        .await
}

pub async fn get_projects(
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<ProjectRaw>, sqlx::Error> {
    sqlx::query_as::<_, ProjectRaw>("select * from projects")
        .fetch_all(conn)
        .await
}

#[derive(Deserialize, Debug)]
pub struct ProjectInsert {
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub deadline: DateTime<Utc>,
    pub budget: f32,
    pub currency_code: String,
}

pub struct ProjectInsertConfig {
    pub user_ids: Vec<String>,
}

impl Dummy<ProjectInsertConfig> for ProjectInsert {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(config: &ProjectInsertConfig, rng: &mut R) -> Self {
        let title: String = Sentence(4..8).fake_with_rng(rng);
        let content: String = Sentence(1..100).fake_with_rng(rng);
        let start = Utc::now() - Duration::days(365);
        let end = Utc::now() + Duration::days(365);
        let deadline: DateTime<Utc> = DateTimeBetween(start, end).fake_with_rng(rng);
        let budget: f32 = Faker.fake_with_rng(rng);
        let currency_code: String = "tn".to_string();

        ProjectInsert {
            user_id: config.user_ids.choose(rng).unwrap().to_string(),
            title,
            content,
            deadline,
            budget,
            currency_code,
        }
    }
}

pub async fn insert_project(
    project_insert: ProjectInsert,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<ProjectRaw, sqlx::Error> {
    sqlx::query_as::<_, ProjectRaw>(
        "
        INSERT INTO projects (user_id, title, content, deadline, budget, currency_code)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        ",
    )
    .bind(project_insert.user_id)
    .bind(project_insert.title)
    .bind(project_insert.content)
    .bind(project_insert.deadline)
    .bind(project_insert.budget)
    .bind(project_insert.currency_code)
    .fetch_one(conn)
    .await
}

pub async fn put_project(
    project_id: i32,
    project_insert: ProjectInsert,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<ProjectRaw, sqlx::Error> {
    sqlx::query_as::<_, ProjectRaw>("
        UPDATE projects SET user_id = $1, title = $2, content = $3, deadline = $4, budget = $5, currency_code = $6
        WHERE id = $7
        RETURNING *
        ")
        .bind(project_insert.user_id)
        .bind(project_insert.title)
        .bind(project_insert.content)
        .bind(project_insert.deadline)
        .bind(project_insert.budget)
        .bind(project_insert.currency_code)
        .bind(project_id)
        .fetch_one(conn)
        .await
}

pub async fn delete_project(
    project_id: i32,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(&project_id)
        .execute(conn)
        .await
        .expect(format!("Failed to delete project from the database {project_id}").as_str());

    Ok(())
}

pub async fn search_products(
    query: String,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<ProjectRaw>, sqlx::Error> {
    sqlx::query_as!(
        ProjectRaw,
        r#"WITH search_query AS (SELECT lower($1) AS raw, websearch_to_tsquery('english', lower($1)) AS ts)
         SELECT p.id, p.user_id, p.title, p.content, p.deadline, p.budget, p.currency_code, p.created_at FROM (
             SELECT p.id, p.user_id, p.title, p.content, p.deadline, p.budget, p.currency_code, p.created_at,
             (ts_rank_cd(p.search_vector, q.ts)) AS fts_rank,
             GREATEST(
                similarity(lower(p.title), q.raw),
                similarity(lower(p.content), q.raw),
                MAX(similarity(lower(t.title), q.raw)),
                MAX(similarity(lower(t.content), q.raw))
             ) AS trgm_similarity
             FROM projects p
             LEFT JOIN tasks t ON p.id = t.project_id
             CROSS JOIN search_query q
             WHERE
                q.ts @@ p.search_vector OR
                lower(p.title) % q.raw OR
                lower(p.content) % q.raw OR
                lower(t.title) % q.raw OR
                lower(t.content) % q.raw
            GROUP BY
                p.id, q.ts, q.raw
            ORDER BY
                fts_rank DESC,
                trgm_similarity DESC
        ) as p"#,
        query
    ).fetch_all(conn).await
}

#[cfg(test)]
mod tests {
    use crate::repository::project::{
        delete_project, get_project_by_id, get_projects, insert_project, put_project, ProjectInsert,
    };
    use chrono::Utc;
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
            currency_code: "TD".to_string(),
        };

        insert_project(project_details, &conn)
            .await
            .expect("Failed to create project");
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn get_all_projects_test(conn: PgPool) {
        let projects = get_projects(&conn).await.unwrap();
        assert_ne!(projects.len(), 0);
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn delete_project_test(conn: PgPool) {
        let projects = delete_project(1, &conn).await;
        assert!(projects.is_ok());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("tasks.sql")))]
    async fn delete_project_with_tasks_test(conn: PgPool) {
        let projects = delete_project(1, &conn).await;
        assert!(projects.is_ok());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("projects.sql")))]
    async fn put_project_test(conn: PgPool) {
        let project_insert = ProjectInsert {
            user_id: "user@gmail.com".to_string(),
            title: "title_updated".to_string(),
            content: "content".to_string(),
            deadline: Utc::now(),
            budget: 10.5,
            currency_code: "TD".to_string(),
        };
        let updated_project = put_project(1, project_insert, &conn).await.unwrap();
        assert_eq!(updated_project.title, "title_updated");
    }
}

