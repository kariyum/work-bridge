use std::fs;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> Result<Pool<Postgres>, std::io::Error> {
    let connection_string = format!(
        "postgres://{}:{}@{}/{}",
        std::env::var("PG_USER").expect("PG_USER env var not set"),
        get_db_password(),
        std::env::var("PG_HOST").expect("PG_HOST env var not set"),
        std::env::var("PG_DBNAME").expect("PG_DBNAME env var not set")
    );
    println!("connection string is {}", connection_string);
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn get_db_password() -> String {
    let password_file = std::env::var("PG_PASSWORD_FILE").expect("PG_PASSWORD_FILE env var not set");
    fs::read_to_string(password_file)
        .expect("Failed to read password file")
        .trim()
        .to_string()
}
