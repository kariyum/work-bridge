use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub discussion_id: i32,
    pub content: String,
    pub sender_id: String,
    // This does not have to be cloned when sent to the Client websocket actor
    // because it doesn't need to know about other receivers maybe?
    pub receivers: Vec<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: WsMessage,
    pub room_id: Uuid,
}