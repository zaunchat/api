use crate::gateway::{ClientPayload, Payload};
use axum::extract::ws;
use rmp_serde as MsgPack;
use serde::Deserialize;
use serde_json as JSON;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    Json,
    MsgPack,
}

impl Default for EncodingFormat {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Debug, Deserialize)]
pub struct SocketClientConfig {
    #[serde(default)]
    pub format: EncodingFormat,
}

impl Default for SocketClientConfig {
    fn default() -> Self {
        Self {
            format: EncodingFormat::Json,
        }
    }
}

impl SocketClientConfig {
    pub fn encode(&self, payload: Payload) -> ws::Message {
        match self.format {
            EncodingFormat::Json => {
                ws::Message::Text(JSON::to_string(&payload).expect("Cannot serialise data (json)"))
            }
            EncodingFormat::MsgPack => ws::Message::Binary(
                MsgPack::encode::to_vec_named(&payload).expect("Cannot serialise data (msgpack)"),
            ),
        }
    }

    pub fn decode(&self, payload: ws::Message) -> Option<ClientPayload> {
        match payload {
            ws::Message::Text(content) => match self.format {
                EncodingFormat::Json => JSON::from_str(&content).ok(),
                _ => None,
            },
            ws::Message::Binary(buf) => match self.format {
                EncodingFormat::MsgPack => MsgPack::from_slice(&buf).ok(),
                _ => None,
            },
            _ => None,
        }
    }
}
