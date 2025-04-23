mod repository;

use std::fs;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

pub mod websocket {
    pub mod lobby;
    pub mod messages;
    pub mod client;
}

pub mod services {
    pub mod token;
}

mod error;
pub mod routes;

use crate::websocket::lobby::Lobby;
use actix::Actor;
use actix_web::web::Data;
use dotenv::dotenv;

fn get_db_password() -> String {
    let password_file = std::env::var("PG_PASSWORD_FILE").expect("PG_PASSWORD_FILE env var not set");
    fs::read_to_string(password_file)
        .expect("Failed to read password file")
        .trim()
        .to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug")); // "info"
    let connection_string = format!(
        "postgres://{}:{}@{}/{}",
        std::env::var("PG_USER").expect("PG_USER env var not set"),
        get_db_password(),
        std::env::var("PG_HOST").expect("PG_HOST env var not set"),
        std::env::var("PG_DBNAME").expect("PG_DBNAME env var not set")
    );
    println!("connection string is {}", connection_string);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let chat_server = Lobby::default().start(); //create and spin up a lobby

    let host = std::env::var("HOST").expect("HOST env var is not set");
    let port = std::env::var("PORT")
        .map(|port| port.parse::<u16>().expect("PORT is not a i32"))
        .expect("PORT env var is not set");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(chat_server.clone())) //register the lobby
            .wrap(Logger::default())
            .service(routes::user_handler::routes())
            .service(routes::project_handler::routes())
            .service(routes::profiles_handler::routes())
            .service(routes::feature_requests_handler::routes())
            .service(routes::comments_handler::routes())
            .service(routes::proposals_handler::routes())
            .service(routes::discussions_handler::routes())
            .service(routes::messages_handler::routes())
            .service(routes::push_events_handler::routes())
            .service(routes::notifications_handler::routes())
    })
    .bind((host, port))?
    .run()
    .await
}
