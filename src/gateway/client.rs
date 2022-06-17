use super::config;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

pub struct SocketClient {
    pub sender: SplitSink<WebSocket, Message>,
    pub receiver: SplitStream<WebSocket>,
    pub config: config::SocektConfig,
}

impl SocketClient {
    pub async fn handle(&mut self) {
        self.on_open().await;

        while let Some(Ok(msg)) = self.receiver.next().await {
            match msg {
                Message::Text(content) => self.on_message(content).await,
                Message::Close(_) => return self.on_close().await,
                _ => {}
            }
        }
    }

    pub async fn on_open(&self) {
        log::debug!("Socket connected");
    }

    pub async fn on_message(&self, content: String) {
        log::debug!("Socket Message: {:?}", content);
    }

    pub async fn on_close(&self) {
        log::debug!("Socked closed");
    }
}
