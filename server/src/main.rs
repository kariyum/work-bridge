mod repository;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug")); // "info"
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/main")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let chat_server = Lobby::default().start(); //create and spin up a lobby

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
