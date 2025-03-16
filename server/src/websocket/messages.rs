use crate::repository::proposal::ProposalStatus;
use crate::websocket::client::Client;
use actix::prelude::Message;
use actix::Addr;
use serde::Serialize;
use uuid::Uuid;

pub type Socket = Addr<Client>;

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct ProposalNotification {
    pub proposal_status: ProposalStatus,
    pub sub_id: String,
    pub proposal_id: i32,
    pub task_id: i32,
    pub project_id: i32,
}

#[derive(Message, Serialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
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
    pub addr: Socket,
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
