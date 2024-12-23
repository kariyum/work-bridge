use crate::websocket::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,      // actor id to actor address
    rooms: HashMap<Uuid, HashSet<Uuid>>,  // room id to list of actor ids
    connections: HashMap<String, Socket>, // user id -> actor address
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            connections: HashMap::new(),
        }
    }
}

impl Lobby {
    /// This function receives a message from a websocket
    /// and broadcasts it to the websocket end
    fn send_message(&self, message: &WsMessage) {
        message.receivers.iter().for_each(|id| {
            println!("In lobby sending message to {:?}", id);
            if let Some(mailbox) = self.connections.get(id) {
                let msg = WsMessage {
                    discussion_id: message.discussion_id,
                    content: message.content.clone(),
                    sender_id: message.sender_id.clone(),
                    receivers: message.receivers.clone(),
                };
                let _ = mailbox.do_send(msg);
            } else {
                println!("Attempted to send message but couldn't find user id.");
            }
        });
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // println!("Disconnect message {:}", &msg);
        // self.connections.remove() TODO remove connections
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    // self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // println!("Connect message handle {:}", &msg);
        self.connections
            .insert(msg.user_id, msg.addr);

        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                // let wsMessage = WsMessage {
                //     disucssion_id: -1,
                //     content: format!("{} just joined!", msg.self_id),
                //     sender_id: msg.self_id.to_string(),
                // };
                // self.send_message(&wsMessage, conn_id)
            });

        // self.sessions.insert(msg.self_id, msg.addr);

        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // println!("Client ActorMessage handle {:}", &msg);
        msg.msg
            .receivers
            .iter()
            .for_each(|client| self.send_message(&msg.msg))
    }
}
