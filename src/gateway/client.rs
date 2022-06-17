use super::{config::SocektConfig, events::*, payload::Payload};
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::collections::HashMap;

pub struct SocketClient {
    pub sender: SplitSink<WebSocket, Message>,
    pub receiver: SplitStream<WebSocket>,
    pub config: SocektConfig,
    pub subscriptions: redis::aio::PubSub,
    pub permissions: HashMap<u64, u64>,
    pub authenticated: bool,
    pub closed: bool,
}

impl SocketClient {
    pub async fn new(stream: WebSocket, config: SocektConfig) -> Self {
        let (sender, receiver) = stream.split();

        Self {
            sender,
            receiver,
            config,
            permissions: HashMap::new(),
            subscriptions: crate::database::redis::pubsub().await,
            authenticated: false,
            closed: false,
        }
    }

    pub async fn handle(&mut self) {
        self.on_open().await;

        while let Some(Ok(msg)) = self.receiver.next().await {
            if self.closed {
                return;
            }

            match msg {
                Message::Text(content) => self.on_message(content).await,
                Message::Close(_) => return self.on_close().await,
                _ => {}
            }
        }
    }

    pub async fn send(&mut self, payload: Payload) {
        self.sender.send(payload.into()).await.unwrap();
    }

    pub async fn close(&mut self) {
        self.closed = true;
        self.sender.close().await.ok();
        self.on_close().await;
    }

    pub async fn on_open(&self) {
        log::debug!("Socket connected");
    }

    pub async fn on_message(&mut self, content: String) {
        let payload = serde_json::from_str::<Payload>(&content);

        if payload.is_err() {
            return self.close().await;
        }

        let payload = payload.unwrap();

        match &payload {
            Payload::Authenticate { .. } => authenticate::run(self, payload).await,
            Payload::Ping => ping::run(self, payload).await,
            _ => {}
        }

        log::debug!("Socket Message: {:?}", content);
    }

    pub async fn on_close(&self) {
        log::debug!("Socked closed");
    }
}
