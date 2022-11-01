use crate::database::redis::publish;
use crate::structures::*;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Empty {
    Default {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        id: i64,
    },
    ServerObject {
        #[serde_as(as = "serde_with::DisplayFromStr")]
        id: i64,
        #[serde_as(as = "serde_with::DisplayFromStr")]
        server_id: i64,
    },
}

impl From<i64> for Empty {
    fn from(id: i64) -> Empty {
        Empty::Default { id }
    }
}

impl From<(i64, i64)> for Empty {
    fn from((id, server_id): (i64, i64)) -> Empty {
        Empty::ServerObject { id, server_id }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "op", content = "d")]
pub enum Payload {
    Pong,
    Authenticated,
    Ready {
        user: User,
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>,
    },
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "op", content = "d")]
pub enum ClientPayload {
    Authenticate { token: String },
    Ping,
}

impl Payload {
    pub async fn to(self, id: i64) {
        publish(id.to_string(), self).await;
    }
}
