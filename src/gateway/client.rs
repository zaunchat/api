use super::{config::SocketConfig, events::*, payload::Payload};
use crate::utils::Permissions;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SocketClient {
    pub sender: SplitSink<WebSocket, Message>,
    pub receiver: SplitStream<WebSocket>,
    pub config: SocketConfig,
    pub subscriptions: redis::aio::PubSub,
    pub permissions: HashMap<u64, Permissions>,
    pub authenticated: bool,
    pub closed: bool,
    pub user_id: u64
}

impl SocketClient {
    pub async fn new(stream: WebSocket, config: SocketConfig) -> Self {
        let (sender, receiver) = stream.split();

        Self {
            sender,
            receiver,
            config,
            permissions: HashMap::new(),
            subscriptions: crate::database::redis::pubsub().await,
            authenticated: false,
            closed: false,
            user_id: 0
        }
    }

    pub async fn handle_outcoming(socket: Arc<Mutex<SocketClient>>) {
        let mut _socket = socket.lock().await;
        let mut stream = _socket.subscriptions.on_message();

        while let Some(msg) = stream.next().await {
            let mut socket = socket.lock().await;

            if socket.closed {
                return;
            }

            let data: String = msg.get_payload().unwrap();
            let target_id: u64 = msg.get_channel_name().parse().unwrap();
            let payload: Payload = serde_json::from_str(&data).unwrap();
            let permissions = socket
                .permissions
                .get(&target_id)
                .unwrap_or(&Permissions::ADMINISTRATOR);

            match payload {
                Payload::MessageCreate(_)
                | Payload::MessageUpdate(_)
                | Payload::MessageDelete(_) => {
                    if !permissions.contains(Permissions::VIEW_CHANNEL) {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                        return;
                    }
                },

                Payload::ChannelDelete(channel) => {
                    if channel.server_id.is_none() {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                    }
                },

                Payload::ServerMemberLeave(data) => {
                    if data.id == socket.user_id {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                    }
                },

                Payload::GroupUserLeave(data) => {
                    if data.id == socket.user_id {
                        socket.subscriptions.unsubscribe(target_id).await.ok();   
                    }
                },

                Payload::ServerDelete(_) => {
                    socket.subscriptions.unsubscribe(target_id).await.ok();
                },

                _ => {}
            }

            socket.send(payload).await;
        }
    }

    pub async fn handle_incoming(socket: Arc<Mutex<SocketClient>>) {
        let mut socket = socket.lock().await;

        socket.on_open().await;

        while let Some(Ok(msg)) = socket.receiver.next().await {
            if socket.closed {
                return;
            }

            match msg {
                Message::Text(content) => socket.on_message(content).await,
                Message::Close(_) => return socket.on_close().await,
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
