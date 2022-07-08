use super::events::*;
use super::payload::{ClientPayload, Payload};
use crate::{structures::User, utils::Permissions};
use axum::extract::ws::{Message, WebSocket};
use fred::clients::RedisClient;
use futures::{stream::SplitSink, SinkExt};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    pub permissions: Mutex<HashMap<i64, Permissions>>,
    pub user: Mutex<Option<User>>,
    pub write: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pub subscriptions: RedisClient,
}

impl Client {
    pub fn from(
        stream: Arc<tokio::sync::Mutex<SplitSink<WebSocket, Message>>>,
        subscriptions: RedisClient,
    ) -> Self {
        Self {
            permissions: Mutex::new(HashMap::new()),
            user: Mutex::new(None),
            write: stream,
            subscriptions,
        }
    }

    pub async fn send(&self, payload: Payload) -> Result<(), axum::Error> {
        self.write.lock().await.send(payload.into()).await
    }

    pub async fn on_message(&self, content: String) {
        let payload = serde_json::from_str::<ClientPayload>(&content);

        if payload.is_err() {
            log::debug!("Socket sent an invalid body");
            return;
        }

        let payload = payload.unwrap();

        match &payload {
            ClientPayload::Authenticate { .. } => authenticate::run(self, payload).await,
            ClientPayload::Ping => ping::run(self, payload).await,
        }

        log::debug!("Socket Message: {:?}", content);
    }
}
