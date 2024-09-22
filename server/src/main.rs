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
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: 10000000000, // TODO update expiration time to millisecondspoch
    };
    // let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret = env::var("SECRET_KEY").unwrap_or("secret".to_string());
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    println!("Token is {:?}", token);
    // let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret = env::var("SECRET_KEY").unwrap_or("secret".to_string());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[get("/")]
async fn hello(request: HttpRequest) -> impl Responder {
    // let cookie = Cookie::build("name", "value")
    // .path("/")
    // .secure(true)
    // .http_only(true)
    // .finish();
    // let mut request = HttpResponse::Ok().body("Hello world!");
    // request.add_cookie(&cookie).unwrap();
    // request
    let x = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()));
    println!("Cookie is {:?}", x);
    HttpResponse::Ok().body("Hello world!")
}

#[options("/login")]
async fn preflight() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .finish()
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct UserRow {
    email: String,
    password: String,
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

#[post("/register")]
async fn register(
    Form(register_request): Form<RegisterRequest>,
    data: web::Data<Pool<Postgres>>,
) -> impl Responder {
    HttpResponse::Ok().body("Register")
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
// async fn login(info: web::Json<LoginRequest>) -> impl Responder {
async fn login(Form(info): Form<LoginRequest>) -> impl Responder {
    let cookie = Cookie::build("Authorization", generate_jwt(&info.email).unwrap())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    let mut request = HttpResponse::Ok().finish();
    request.add_cookie(&cookie).unwrap();
    request
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
