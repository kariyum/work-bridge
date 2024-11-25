use actix_web::{
    cookie::Cookie,
    post,
    web::{self, Form},
    HttpResponse, Responder,
};

use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};
use sqlx::pool::PoolConnection;
use crate::repository::user::UserRow;

#[derive(Deserialize, Debug)]
struct RegisterRequest {
    email: String,
    password: String,
    role: String,
    first_name: String,
    last_name: String,
}

use super::token::generate_cookie;

#[post("/register")]
pub async fn register(
    Form(register_request): Form<RegisterRequest>,
    data: web::Data<Pool<Postgres>>,
) -> impl Responder {
    // check if user exists, return error
    // else, insert user into database
    // return 200 status code and set cookie
    let mut client = data
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");

    let _ = insert_user(&register_request, client)
        .await
        .expect("Failed to insert user");

    let cookie = generate_cookie(&register_request.email, &register_request.role).unwrap();
    let mut request = HttpResponse::Ok().finish();
    request.add_cookie(&cookie).unwrap();

    println!("Register request is {:?}", register_request);

    request
}

async fn insert_user(register_request: &RegisterRequest, mut client: PoolConnection<Postgres>) -> Result<Option<UserRow>, sqlx::Error> {
    let sql_query = include_str!("./sql/insert_user.sql");
    let mut hasher = DefaultHasher::new();
    register_request.password.hash(&mut hasher);
    let hashed_password = hasher.finish().to_string();
    let row = sqlx::query_as::<_, UserRow>(sql_query)
        .bind(&register_request.email)
        .bind(&hashed_password)
        .bind(&register_request.role)
        .bind(&register_request.first_name)
        .bind(&register_request.last_name)
        .fetch_optional(&mut *client)
        .await
        .expect("Failed to insert user into database");

    Ok(row)
}
