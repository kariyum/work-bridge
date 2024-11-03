use crate::websocket::lobby::Lobby;

use crate::messages::repo::{create_message, MessageCreate};
use crate::websocket::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{fut, ActorContext, ActorFuture, ActorFutureExt, ContextFutureSpawner, Message, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web::web;
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::PgPool;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
    pgpool: web::Data<PgPool>,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct ClientMessage {
    disucssion_id: Option<i32>,
    content: String,
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
struct ClientMessageResponse {
    disucssion_id: i32,
    content: String,
    sender_id: i32,
}

impl WsConn {
    pub fn new(
        room: Uuid,
        lobby: Addr<Lobby>,
        pgpool: web::Data<PgPool>,
        user_id: String,
    ) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby,
            pgpool: pgpool,
            user_id: user_id,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WsConn is started");

        // register heartbeats
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("WsConn is stopped");
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("Close received: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                println!("Recieved a message from client: {:?}", s);
                let client_message = serde_json::from_str::<ClientMessage>(&s);
                if let Err(err) = client_message {
                    let response = ClientMessage {
                        disucssion_id: Option::None,
                        content: err.to_string(),
                    };
                    ctx.text(serde_json::to_string(&response).unwrap());
                    println!(
                        "Failed to deserialize websocket message to ClientMessage {:?}",
                        err
                    );
                    return;
                }
                let client_message = client_message.unwrap();
                let pool = self.pgpool.clone();
                let user_id = self.user_id.clone();
                let client_actor_message = ClientActorMessage {
                    id: self.id,
                    msg: WsMessage {
                        disucssion_id: client_message.disucssion_id.unwrap(),
                        content: client_message.content.clone(),
                        sender_id: user_id.clone(),
                    },
                    room_id: self.room,
                };
                async move {
                    let message_create = MessageCreate {
                        discussion_id: client_message.disucssion_id.unwrap(),
                        content: client_message.content.clone(),
                    };
                    let client = pool
                        .acquire()
                        .await
                        .expect("Failed to acquire a Postgres connection from the pool");
                    create_message(user_id, message_create, client).await;
                }
                .into_actor(self)
                .spawn(ctx);
                self.lobby_addr.do_send(client_actor_message);
            }
            Err(e) => std::panic::panic_any(e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        // messages coming from Lobby actor are handled here
        println!("Sending message to client: {:?}", msg);
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}
