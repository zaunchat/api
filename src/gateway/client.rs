use super::events::*;
use super::payload::Payload;
use crate::{structures::User, utils::Permissions};
use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::Mutex;

pub enum Subscription {
    Add(Vec<u64>),
    Remove(Vec<u64>),
    None,
}

pub struct Client {
    pub permissions: HashMap<u64, Permissions>,
    pub user: Option<User>,
    pub write: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pub subscriptions: Subscription,
}

impl Client {
    pub fn from(stream: Arc<Mutex<SplitSink<WebSocket, Message>>>) -> Self {
        Self {
            permissions: HashMap::new(),
            user: None,
            write: stream,
            subscriptions: Subscription::None,
        }
    }

    pub async fn send(&self, payload: Payload) -> Result<(), axum::Error> {
        self.write.lock().await.send(payload.into()).await
    }

    pub async fn on_message(&mut self, content: String) {
        let payload = serde_json::from_str::<Payload>(&content);

        if payload.is_err() {
            log::debug!("Socket sent an invalid body");
            return;
        }

        let payload = payload.unwrap();

        match &payload {
            Payload::Authenticate { .. } => authenticate::run(self, payload).await,
            Payload::Ping => ping::run(self, payload).await,
            _ => {}
        }

        log::debug!("Socket Message: {:?}", content);
    }
}
