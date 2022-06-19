use super::{config::SocketConfig, events::*, payload::Payload};
use crate::utils::Permissions;
use crate::{structures::User, utils::Ref};
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
    pub user: Option<User>,
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
            user: None,
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
            let user = socket.user.as_ref().unwrap();
            let payload: Payload = serde_json::from_str(&data).unwrap();
            let p = socket
                .permissions
                .get(&target_id)
                .unwrap_or(&Permissions::ADMINISTRATOR);

            match &payload {
                Payload::MessageCreate(_)
                | Payload::MessageUpdate(_)
                | Payload::MessageDelete(_) => {
                    if !p.contains(Permissions::VIEW_CHANNEL) {
                        return;
                    }
                }

                Payload::ChannelDelete(channel) => {
                    if channel.server_id.is_none() {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                    }
                }

                Payload::ChannelUpdate(channel) => {
                    let server = if let Some(server_id) = channel.server_id {
                        Some(server_id.server().await.unwrap())
                    } else {
                        None
                    };

                    let p = Permissions::fetch_cached(user, server.as_ref(), channel.into())
                        .await
                        .unwrap();

                    socket.permissions.insert(channel.id, p);
                }

                Payload::ServerMemberUpdate(member) => {
                    let p = Permissions::fetch(user, member.server_id.into(), None)
                        .await
                        .unwrap();
                    socket.permissions.insert(member.server_id, p);
                }

                Payload::ServerMemberLeave(data) => {
                    if data.id == user.id {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                    }
                }

                Payload::GroupUserLeave(data) => {
                    if data.id == user.id {
                        socket.subscriptions.unsubscribe(target_id).await.ok();
                    }
                }

                Payload::ServerDelete(_) => {
                    socket.subscriptions.unsubscribe(target_id).await.ok();
                }

                Payload::ServerCreate(server) => {
                    socket.subscriptions.subscribe(server.id).await.ok();
                }

                Payload::ChannelCreate(channel) => {
                    socket.subscriptions.subscribe(channel.id).await.ok();
                }
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
