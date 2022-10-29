use crate::gateway::{ClientPayload, Payload};
use axum::extract::ws;
use rmp as MsgPack;
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
                ws::Message::Text(JSON::to_string(&payload).expect("Cannot stringify json"))
            }
            EncodingFormat::MsgPack => {
                let mut buf = Vec::new();
                MsgPack::encode::write_str(&mut buf, &JSON::to_string(&payload).unwrap()).unwrap();
                ws::Message::Binary(buf)
            }
        }
    }

    pub fn decode(&self, payload: ws::Message) -> Option<ClientPayload> {
        match payload {
            ws::Message::Text(content) => match self.format {
                EncodingFormat::Json => JSON::from_str(&content).ok(),
                _ => None,
            },
            ws::Message::Binary(buf) => match self.format {
                EncodingFormat::MsgPack => {
                    let mut content = Vec::new();

                    MsgPack::decode::read_str(&mut &buf[..], &mut content).ok();

                    JSON::from_slice(&content).ok()
                }
                _ => None,
            },
            _ => None,
        }
    }
}
