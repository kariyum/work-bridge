pub mod repository;
pub mod services {
    pub mod token;
    pub mod database;
}
pub mod routes;

pub mod websocket {
    pub mod client;
    pub mod lobby;
    pub mod messages;
}

pub mod error;