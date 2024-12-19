use chrono::{DateTime, Utc};
use futures_util::{stream, StreamExt};
use serde::Serialize;
use sqlx::{Executor, PgPool, Postgres};

#[derive(sqlx::FromRow, Serialize)]
pub struct RawProfile {
    user_id: String,
    skills: Vec<String>,
    birthdate: DateTime<Utc>,
    phone: String,
}

pub async fn read_profile(user_id: String, conn: impl Executor<'_, Database=Postgres>) -> Result<Vec<RawProfile>, sqlx::Error> {
    sqlx::query_as::<_, RawProfile>("SELECT * FROM profiles WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(conn).await
}

pub struct CreateProfile {
    pub user_id: String,
    pub skills: Vec<String>,
    pub birthdate: DateTime<Utc>,
    pub phone: String,
}

pub async fn insert_profile(create_profile: CreateProfile, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO profiles (user_id, skills, birthdate, phone)
                     VALUES ($1, $2, $3, $4)
                     ON CONFLICT (user_id)
                     DO UPDATE
                        SET skills = EXCLUDED.skills, birthdate = EXCLUDED.birthdate, phone = EXCLUDED.phone")
        .bind(create_profile.user_id)
        .bind(create_profile.skills)
        .bind(create_profile.birthdate)
        .bind(create_profile.phone)
        .execute(conn)
        .await
        .map(|_| {})
}
