use axum::extract::ws::Message;
use serde::{Deserialize, Serialize};
use crate::structures::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Payload {
    Authenticate { token: String },
    Ping,
    Authenticated,
    Ready { user: User, servers: Vec<Server>, channels: Vec<Channel> },
    Pong,
}

impl From<Payload> for Message {
    fn from(payload: Payload) -> Message {
        Message::Text(serde_json::to_string(&payload).unwrap())
    }
}