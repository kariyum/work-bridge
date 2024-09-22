use actix_cors::Cors;
use actix_web::{
    cookie::Cookie,
    get,
    http::header,
    middleware::Logger,
    options, post,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub mod security {
    pub mod token;
    pub mod login;
    pub mod register;
}

use security::token::validate_jwt;
use security::login::{login, preflight};
use security::register::register;

#[derive(Serialize, sqlx::FromRow)]
struct UserRow {
    email: String,
    password: String,
}

#[get("/")]
async fn hello(request: HttpRequest) -> impl Responder {
    let x = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()));
    println!("Cookie is {:?}", x);
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("users")]
async fn get_users(data: web::Data<PgPool>) -> impl Responder {
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/main_db")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:5173")
        //     .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
        //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //     .allowed_header(header::CONTENT_TYPE)
        //     .max_age(3600);
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(login)
            .service(hello)
            .service(preflight)
            .service(register)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
