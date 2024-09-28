use actix_web::{
    get,
    web::{self}, HttpResponse, Responder,
};
use serde::Serialize;
use sqlx::PgPool;


#[derive(Serialize, sqlx::FromRow)]
struct UserRow {
    email: String,
    password: String,
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
