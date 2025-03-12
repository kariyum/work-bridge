mod repository;

use crate::services::token::validate_jwt;
use actix_cors::Cors;
use actix_web::{
    dev::Service,
    get,
    middleware::Logger,
    rt,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[get("/")]
async fn hello(request: HttpRequest) -> impl Responder {
    let x = request
        .cookie("Authorization")
        .map(|token| validate_jwt(token.value()));
    println!("Cookie is {:?}", x);
    HttpResponse::Ok().body("Hello world!")
}
use actix_web::Error;
pub mod websocket {
    pub mod lobby;
    pub mod messages;
    pub mod ws;
}

pub mod services {
    pub mod token;
}

pub mod tasks {
    pub mod repo;
}
mod error;
pub mod routes;

use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

use crate::websocket::lobby::Lobby;
use crate::websocket::ws::WsConn;
use actix::{Actor, Addr};
use actix_web::{web::Data, web::Payload};

#[get("/chat")]
async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    lobby_addr: Data<Addr<Lobby>>,
    pgpool: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .cookie("Authorization")
        .and_then(|cookie| validate_jwt(cookie.value()).ok())
        .map(|claims| claims.sub);

    if user_id.is_none() {
        Ok(HttpResponse::Unauthorized().finish())
    } else {
        let ws = WsConn::new(lobby_addr.get_ref().clone(), pgpool, user_id.unwrap());
        let resp = actix_web_actors::ws::start(ws, &req, stream)?;
        Ok(resp)
    }
}

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

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
            .route("/echo", web::get().to(echo))
            .service(start_connection)
            .service(tasks::repo::create_task)
            .service(tasks::repo::get_tasks)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
