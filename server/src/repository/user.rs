use serde::Deserialize;
use sqlx::{Error, Executor, PgPool, Postgres};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct UserRow {
    pub email: String,
    pub role: String,
}

pub async fn get_user_by_credentials(email: String, password: String, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<UserRow>, Error> {
    let hashed_password = hash_password(&password);
    get_user(email, hashed_password, conn).await
}

pub async fn get_user(email: String, password: String, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<UserRow>, Error> {
    let user = sqlx::query_as::<_, UserRow>("SELECT email, password, role FROM users WHERE email = $1 AND hashed_password = $2")
        .bind(&email)
        .bind(&password)
        .fetch_optional(conn)
        .await;
    user
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub email: String,
    password: String,
    pub role: String,
    first_name: String,
    last_name: String,
}

pub async fn insert_user(register_request: &RegisterRequest, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    let hashed_password = hash_password(&register_request.password);
    sqlx::query("
            INSERT INTO users (email, hashed_password, role, first_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
        "
    )
        .bind(&register_request.email)
        .bind(&hashed_password)
        .bind(&register_request.role)
        .bind(&register_request.first_name)
        .bind(&register_request.last_name)
        .execute(conn)
        .await
        .expect("Failed to insert user into database");
    Ok(())
}

fn hash_password(password: &String) -> String {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish().to_string()
}

mod tests {
    use super::*;

    #[sqlx::test(fixtures(path = "fixtures", scripts("users.sql")))]
    async fn read_user_test(pool: PgPool) -> sqlx::Result<()> {
        let row = get_user("test@gmail.com".to_string(), "password".to_string(), &pool).await;

        assert_eq!(row?.unwrap().email, "test@gmail.com");

        Ok(())
    }

    #[sqlx::test]
    async fn insert_user_test(pool: PgPool) -> sqlx::Result<()> {
        let register_request = RegisterRequest {
            email: "test_email@gmail.com".to_string(),
            password: "password".to_string(),
            role: "role".to_string(),
            first_name: "first".to_string(),
            last_name: "last".to_string(),
        };
        let _ = insert_user(&register_request, &pool).await?;
        let user = get_user_by_credentials("test_email@gmail.com".to_string(), "password".to_string(), &pool)
            .await
            .expect("Failed to get user");
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, "test_email@gmail.com");
        Ok(())
    }
}
