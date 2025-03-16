use crate::services::token::Claims;
use crate::websocket::lobby::Lobby;
use crate::websocket::client::Client;
use actix::Addr;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::{get, scope, Data, Payload};
use actix_web::{Error, HttpRequest, HttpResponse};
use sqlx::PgPool;

async fn start_connection(
    claims: Claims,
    req: HttpRequest,
    stream: Payload,
    lobby_addr: Data<Addr<Lobby>>,
    pgpool: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let ws = Client::new(lobby_addr.get_ref().clone(), pgpool, claims.sub);
    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    Ok(resp)
}

pub fn routes() -> impl HttpServiceFactory {
    scope("/push_events").route("", get().to(start_connection))
}
