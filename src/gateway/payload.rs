use crate::database::redis::publish;
use crate::structures::*;
use crate::utils::Snowflake;
use serde::{Deserialize, Serialize};

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Empty {
    Default { id: Snowflake },
}

impl From<Snowflake> for Empty {
    fn from(id: Snowflake) -> Empty {
        Empty::Default { id }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "op", content = "d")]
pub enum Payload {
    Pong,
    Authenticated,
    Ready {
        user: User,
        users: Vec<User>,
        channels: Vec<Channel>,
    },
    ChannelCreate(Channel),
    ChannelDelete(Empty),
    ChannelUpdate(Channel),
    MessageCreate(Message),
    MessageDelete(Empty),
    MessageUpdate(Message),
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
