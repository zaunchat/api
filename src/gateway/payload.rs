use crate::structures::*;
use axum::extract::ws;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Empty {
    id: u64
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Payload {
    Authenticate {
        token: String,
    },
    Ping,
    Authenticated,
    Ready {
        user: User,
        servers: Vec<Server>,
        channels: Vec<Channel>,
    },
    Pong,
    MessageCreate(Message),
    MessageDelete(Empty),
    MessageUpdate(Message)
}

impl From<Payload> for ws::Message {
    fn from(payload: Payload) -> ws::Message {
        ws::Message::Text(serde_json::to_string(&payload).unwrap())
    }
}
