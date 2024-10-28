use std::future::ready;

use actix_cors::Cors;
use actix_web::{
    dev::{Service, ServiceResponse}, get, middleware::Logger, rt, web::{self}, App, HttpRequest, HttpResponse, HttpServer, Responder
};
use sqlx::postgres::PgPoolOptions;

pub mod security {
    pub mod login;
    pub mod register;
    pub mod token;
}

pub mod repo {
    pub mod user;
    pub mod posts;
}

pub mod messaging {
    pub mod discussions;
}
use messaging::discussions::{get_discussions, post_discussions};

use repo::user::get_users;
use security::login::{login, preflight};
use security::register::register;
use security::token::validate_jwt;
use repo::posts::get_posts;

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
    pub mod ws;
    pub mod messages;
    pub mod lobby;
}
use websocket::ws;

pub mod project {
    pub mod repo;
    pub mod service;
    pub mod route;
}

use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

use crate::ws::WsConn;
use crate::websocket::lobby::Lobby;
use actix::{Actor, Addr};
use actix_web::{web::Data, web::Path, web::Payload};
use uuid::Uuid;

#[get("/chat/{group_id}")]
async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<Uuid>,
    lobby_addr: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let room = path.into_inner();
    let ws = WsConn::new(
        room,
        lobby_addr.get_ref().clone(),
    );

    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    Ok(resp)
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/main")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let chat_server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone())) //register the lobby
            .wrap(Logger::default())
            // .service(
            //     web::scope("")
            //         .wrap_fn(|req, srv| {
            //             let is_authorized = req
            //                 .cookie("Authorization")
            //                 .map(|cookie| validate_jwt(cookie.value()).ok())
            //                 .flatten();

            //             // let fut = async move {
            //             //     if is_authorized.is_some() {
            //             //         let response = HttpResponse::Unauthorized().finish();
            //             //         let (req, _pl) = req.into_parts();
            //             //         return Ok(ServiceResponse::new(req, response));
            //             //     }
            //             //     let res = srv.call(req).await?;
            //             //     Ok(res.into())
            //             // };
            //             // Box::pin(fut)
            //             // let short_circuit = HttpResponse::Unauthorized().finish().map_into_boxed_body();
            //             let short_circuit_response: std::future::Ready<Result<ServiceResponse, actix_web::Error>> = {
            //                 if is_authorized.is_some() {
            //                     let response = HttpResponse::Unauthorized().finish();
            //                     let (req, _pl) = req.into_parts();
            //                     ready(Ok(ServiceResponse::new(req, response).map_into_boxed_body()))
            //                 } else {
            //                     let fut = srv.call(req);
            //                     Ok(fut.await?.into())
            //                 }
            //             };
            //             Box::pin(short_circuit_response)
            //             // Box::pin(async move { Ok(fut.await?) })
            //         })
            //         .wrap(Logger::new("%a %{User-Agent}i"))
            //         .service(hello)
            //         .service(preflight)
            //         .service(register)
            //         .service(get_users),
            // )
            .service(login)
            .service(hello)
            .service(preflight)
            .service(register)
            .service(get_users)
            .service(get_posts)
            .service(get_discussions)
            .route("/echo", web::get().to(echo))
            .service(start_connection)
            .service(project::repo::create_project)
            .service(project::repo::get_projects)
            .service(project::repo::delete_project)
            .service(project::repo::get_project)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
