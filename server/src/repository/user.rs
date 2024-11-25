use actix_web::Responder;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Executor, PgPool, Postgres, Row};
use std::future::Future;
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct UserRow {
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

pub async fn get_user(email: String, password: String, conn: impl Executor<'_, Database=Postgres>) -> Result<Option<UserRow>, Error> {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    let hashed_password = hasher.finish().to_string();
    let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1 AND hashed_password = $2")
        .bind(&email)
        .bind(&hashed_password)
        .fetch_optional(conn)
        .await;
    let copied = user.as_ref();
    println!("Getting user {:?}", copied.unwrap());
    user
}
#[derive(Deserialize, Debug)]
struct RegisterRequest {
    email: String,
    password: String,
    role: String,
    first_name: String,
    last_name: String,
}
pub async fn insert_user(register_request: &RegisterRequest, conn: impl Executor<'_, Database=Postgres>) -> Result<(), sqlx::Error> {
    // let sql_query = include_str!("./sql/insert_user.sql");
    let mut hasher = DefaultHasher::new();
    register_request.password.hash(&mut hasher);
    let hashed_password = hasher.finish().to_string();
    let x = sqlx::query("
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
    println!("Inserting user {:?}", register_request.email);
    println!("PgQueryResult {:?}", x);
    Ok(())
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
        let user = get_user("test_email@gmail.com".to_string(), "password".to_string(), &pool)
            .await
            .expect("Failed to get user");
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, "test_email@gmail.com");
        Ok(())
    }
}
