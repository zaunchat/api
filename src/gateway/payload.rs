use crate::database::redis::publish;
use crate::structures::*;
use crate::utils::Snowflake;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Empty {
    Default { id: Snowflake },
    ServerObject { id: Snowflake, server_id: Snowflake },
}

impl From<Snowflake> for Empty {
    fn from(id: Snowflake) -> Empty {
        Empty::Default { id }
    }
}

impl From<(Snowflake, Snowflake)> for Empty {
    fn from((id, server_id): (Snowflake, Snowflake)) -> Empty {
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
    pub async fn to(self, id: Snowflake) {
        publish(id.to_string(), self).await;
    }
}
