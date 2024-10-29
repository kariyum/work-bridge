use std::future::Future;

use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::Serialize;
use sqlx::{pool::PoolConnection, PgPool, Postgres};

#[derive(Serialize, sqlx::FromRow)]
pub struct UserRow {
    pub email: String,
    pub hashed_password: String,
    pub role: String
}

#[get("users")]
pub async fn get_users(data: web::Data<PgPool>) -> impl Responder {
    let mut client = data
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    // let stmt = include_str!("../../migrations/create_users.sql");
    // sqlx::query(stmt).execute(&mut *client).await.expect("Failed to create users table");
    let users = sqlx::query_as::<_, UserRow>("SELECT * FROM users")
        .fetch_all(&mut *client)
        .await
        .expect("Failed to fetch users");

    HttpResponse::Ok().json(users)
}

pub async fn get_user(email: String, password: String, mut client: PoolConnection<Postgres>) -> Option<UserRow> {
    let users: Option<UserRow> = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1 AND hashed_password = $2")
        .bind(&email)
        .bind(&password)
        .fetch_optional(&mut *client)
        .await
        .expect("Failed to fetch users");
    users
}