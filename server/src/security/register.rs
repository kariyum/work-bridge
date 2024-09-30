use actix_web::{
    cookie::Cookie, post, web::{self, Form}, HttpResponse, Responder
};

use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher};

#[derive(Deserialize, Debug)]
struct RegisterRequest {
    email: String,
    password: String,
    role: String,
    name: String,
    surname: String,
}

use super::token::generate_cookie;

#[post("/register")]
pub async fn register(
    Form(register_request): Form<RegisterRequest>,
    data: web::Data<Pool<Postgres>>,
) -> impl Responder {
    // check if user exists
    // if user exists, return error
    // else, insert user into database
    // return 200 status code and set cookie
    let sql_query = include_str!("./sql/insert_user.sql");
    let mut client = data
        .acquire()
        .await
        .expect("Failed to acquire a Postgres connection from the pool");
        let mut hasher = DefaultHasher::new();
        register_request.password.hash(&mut hasher);
        let hashed_password = hasher.finish().to_string();
        let query = sqlx::query(sql_query)
        .bind(&register_request.email)
        .bind(&hashed_password)
        .bind(&register_request.role)
        .bind(&register_request.name)
        .bind(&register_request.surname)
        .execute(&mut *client)
        .await
        .expect("Failed to insert user into database");

    let cookie = generate_cookie(&register_request.email).unwrap();
    let mut request = HttpResponse::Ok().finish();
    request.add_cookie(&cookie).unwrap();
    
    println!("Register request is {:?}", register_request);
    println!("Query result is {:?}", query);

    request
}
