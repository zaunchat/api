use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;

use futures::stream::SplitSink;

pub struct Sender(Arc<Mutex<SplitSink<WebSocket, Message>>>);

impl Clone for Sender {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Sender {
    pub fn new(stream: SplitSink<WebSocket, Message>) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }

    pub async fn send(&self, message: Message) -> Result<(), axum::Error> {
        self.0.lock().await.send(message).await
    }
}
