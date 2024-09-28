use std::future::ready;

use actix_cors::Cors;
use actix_web::{
    dev::{Service, ServiceResponse},
    get,
    middleware::Logger,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/main")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
