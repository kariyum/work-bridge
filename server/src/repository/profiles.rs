use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Executor, Postgres};

#[derive(Serialize)]
pub struct RawProfile {
    user_id: String,
    first_name: String,
    last_name: String,
    role: String,
    skills: Option<Vec<String>>,
    github_link: Option<String>,
    linkedin_link: Option<String>,
    portfolio_link: Option<String>,
    created_at: Option<DateTime<Utc>>,
    bio: Option<String>,
}
pub async fn read_profile(
    user_id: String,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<Option<RawProfile>, sqlx::Error> {
    sqlx::query_as!(
        RawProfile,
        r#"SELECT
            users.email AS user_id,
            users.role,
            first_name,
            last_name,
            profiles.skills AS "skills: Option<Vec<String>>",
            github_link,
            linkedin_link,
            portfolio_link,
            profiles.created_at AS "created_at: Option<DateTime<Utc>>",
            bio
        FROM users
        LEFT JOIN profiles ON profiles.user_id = users.email
        WHERE users.email = $1"#,
        user_id
    )
    .fetch_optional(conn)
    .await
}

pub struct CreateProfile {
    pub user_id: String,
    pub skills: Vec<String>,
    pub bio: Option<String>,
    pub linkedin_link: Option<String>,
    pub github_link: Option<String>,
    pub portfolio_link: Option<String>,
}

pub async fn upsert_profile(
    create_profile: CreateProfile,
    conn: impl Executor<'_, Database = Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO profiles (user_id, skills, bio, linkedin_link, github_link, portfolio_link)
                     VALUES ($1, $2, $3, $4, $5, $6)
                     ON CONFLICT (user_id)
                     DO UPDATE
                        SET skills = EXCLUDED.skills, linkedin_link = EXCLUDED.linkedin_link, github_link = EXCLUDED.github_link, portfolio_link = EXCLUDED.portfolio_link, bio = EXCLUDED.bio",
        create_profile.user_id,
        &create_profile.skills,
        create_profile.bio,
        create_profile.linkedin_link,
        create_profile.github_link,
        create_profile.portfolio_link
    )
        .execute(conn)
        .await
        .map(|_| {})
}

#[cfg(test)]
mod test {
    use crate::repository::profiles::read_profile;
    use sqlx::PgPool;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("users.sql")))]
    async fn insert_task_test(pg_pool: PgPool) {
        let profile = read_profile(String::from("test@gmail.com"), &pg_pool)
            .await
            .unwrap();
        assert!(profile.is_some())
    }
}
