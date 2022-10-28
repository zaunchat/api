use super::payload::{ClientPayload, Payload};
use super::{events::*, Empty};
use crate::{
    structures::User,
    utils::{Permissions, Ref},
};
use axum::extract::ws::{Message, WebSocket};
use fred::{clients::SubscriberClient, interfaces::PubsubInterface};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

lazy_static! {
    static ref DEFAULT_PERMISSION: Permissions = Permissions::all();
}

pub struct Client {
    pub permissions: Mutex<HashMap<i64, Permissions>>,
    pub user: Mutex<Option<User>>,
    pub write: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    pub subscriptions: SubscriberClient,
}

impl Client {
    pub fn new(stream: SplitSink<WebSocket, Message>, subscriptions: SubscriberClient) -> Self {
        Self {
            permissions: Mutex::new(HashMap::new()),
            user: Mutex::new(None),
            write: Arc::new(Mutex::new(stream)),
            subscriptions,
        }
    }

    pub async fn send(&self, payload: Payload) -> Result<(), axum::Error> {
        self.write.lock().await.send(payload.into()).await
    }

    pub async fn handle_outgoing(&self) {
        while let Some((channel, payload)) = self.subscriptions.on_message().next().await {
            let target_id: i64 = channel.parse().unwrap();
            let user = self.user.lock().await;
            let user = user.as_ref().unwrap();

            let payload: Payload = serde_json::from_str(&payload.as_string().unwrap()).unwrap();
            let mut permissions = self.permissions.lock().await;
            let p = permissions.get(&target_id).unwrap_or(&DEFAULT_PERMISSION);

            match &payload {
                Payload::MessageCreate(_)
                | Payload::MessageUpdate(_)
                | Payload::MessageDelete(_) => {
                    if !p.contains(Permissions::VIEW_CHANNEL) {
                        continue;
                    }
                }

                Payload::ChannelDelete(Empty::Default { id }) => {
                    self.subscriptions.unsubscribe(id.to_string()).await.ok();
                }

                Payload::ChannelUpdate(channel) => {
                    let server = if let Some(server_id) = channel.server_id {
                        Some(server_id.server(None).await.unwrap())
                    } else {
                        None
                    };

                    let p = Permissions::fetch_cached(user, server.as_ref(), channel.into())
                        .await
                        .unwrap();

                    permissions.insert(channel.id, p);
                }

                Payload::ServerMemberUpdate(member) => {
                    if member.id == user.id {
                        let p = Permissions::fetch(user, member.server_id.into(), None)
                            .await
                            .unwrap();
                        permissions.insert(member.server_id, p);
                    }
                }

                Payload::ServerMemberLeave(Empty::ServerObject { id, .. }) => {
                    if *id == user.id {
                        self.subscriptions
                            .unsubscribe(target_id.to_string())
                            .await
                            .ok();
                    }
                }

                Payload::ServerDelete(_) => {
                    self.subscriptions
                        .unsubscribe(target_id.to_string())
                        .await
                        .ok();
                }

                Payload::ServerCreate(server) => {
                    self.subscriptions
                        .subscribe(server.id.to_string())
                        .await
                        .ok();
                }

                Payload::ChannelCreate(channel) => {
                    self.subscriptions
                        .subscribe(channel.id.to_string())
                        .await
                        .ok();

                    if !channel.in_server() {
                        permissions.insert(
                            channel.id,
                            Permissions::fetch_cached(user, None, channel.into())
                                .await
                                .unwrap(),
                        );
                    }
                }
                Payload::UserUpdate(u) => {
                    // Newly friend, blocked, request
                    if u.id != target_id && u.id != user.id {
                        self.subscriptions.subscribe(u.id.to_string()).await.ok();
                    }
                }
                _ => {}
            }

            if self.send(payload).await.is_err() {
                break;
            }
        }
    }

    pub async fn handle_incoming(&self, mut receiver: SplitStream<WebSocket>) {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(content) => {
                    let payload = serde_json::from_str::<ClientPayload>(&content);

                    if payload.is_err() {
                        log::debug!("Socket sent an invalid body");
                        break;
                    }

                    let payload = payload.unwrap();

                    match &payload {
                        ClientPayload::Authenticate { .. } => {
                            authenticate::run(self, payload).await
                        }
                        ClientPayload::Ping => ping::run(self, payload).await,
                    }

                    if self.user.lock().await.is_none() {
                        log::debug!("Socket did not authenticate with valid token");
                        break;
                    }

                    log::debug!("Socket Message: {:?}", content);
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}
