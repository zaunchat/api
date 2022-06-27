use crate::structures::*;
use crate::utils::snowflake;
use axum::extract::ws;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Empty {
    Default {
        #[serde_as(as = "snowflake::json::ID")]
        id: u64,
    },
    ServerObject {
        #[serde_as(as = "snowflake::json::ID")]
        id: u64,
        #[serde_as(as = "snowflake::json::ID")]
        server_id: u64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Payload {
    Authenticate {
        token: String,
    },
    Ping,
    Authenticated,
    Ready {
        user: User,
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>,
    },
    Pong,
    ChannelCreate(Channel),
    ChannelDelete(Empty),
    ChannelUpdate(Channel),
    MessageCreate(Message),
    MessageDelete(Empty),
    MessageUpdate(Message),
    RoleCreate(Role),
    RoleDelete(Empty),
    RoleUpdate(Role),
    ServerCreate(Server),
    ServerDelete(Empty),
    ServerMemberJoin(Member),
    ServerMemberLeave(Empty),
    ServerMemberUpdate(Member),
    ServerUpdate(Server),
    UserUpdate(User),
}

impl From<Payload> for ws::Message {
    fn from(payload: Payload) -> ws::Message {
        ws::Message::Text(serde_json::to_string(&payload).unwrap())
    }
}
