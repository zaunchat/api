use super::{
    client::{Client, Subscription},
    Payload,
};
use crate::{
    gateway::Empty,
    utils::{Permissions, Ref},
};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn upgrade(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle)
}

async fn handle(ws: WebSocket) {
    let (sender, mut receiver) = ws.split();
    let sender = Arc::new(Mutex::new(sender));

    let mut subscriptions = crate::database::redis::pubsub().await;
    let mut client = Client::from(sender);

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(content) = msg {
            client.on_message(content).await;

            if client.user.is_some() {
                break;
            } else {
                log::debug!("Socket did not authenticate with valid token");
                return;
            }
        }
    }

    let process = tokio::spawn(async move {
        loop {
            match &client.subscriptions {
                Subscription::Add(ids) => {
                    for id in ids {
                        log::debug!("Subscribe to: {}", id);
                        subscriptions.subscribe(id).await.unwrap();
                    }

                    client.subscriptions = Subscription::None;
                }
                Subscription::Remove(ids) => {
                    for id in ids {
                        log::debug!("Unsubscribe from: {}", id);
                        subscriptions.unsubscribe(id).await.unwrap();
                    }
                    client.subscriptions = Subscription::None;
                }
                Subscription::None => {}
            }

            match subscriptions.on_message().next().await {
                Some(msg) => {
                    let target_id: u64 = msg.get_channel_name().parse().unwrap();
                    let user = client.user.as_ref().unwrap();
                    let payload: Payload =
                        serde_json::from_str(&msg.get_payload::<String>().unwrap()).unwrap();

                    let p = client
                        .permissions
                        .get(&target_id)
                        .unwrap_or(&Permissions::ADMINISTRATOR);

                    match &payload {
                        Payload::MessageCreate(_)
                        | Payload::MessageUpdate(_)
                        | Payload::MessageDelete(_) => {
                            if p.has(Permissions::VIEW_CHANNEL).is_err() {
                                continue;
                            }
                        }

                        Payload::ChannelDelete(Empty::Default { id }) => {
                            client.subscriptions = Subscription::Remove(vec![*id]);
                        }

                        Payload::ChannelUpdate(channel) => {
                            let server = if let Some(server_id) = channel.server_id {
                                Some(server_id.server(None).await.unwrap())
                            } else {
                                None
                            };

                            let p =
                                Permissions::fetch_cached(user, server.as_ref(), channel.into())
                                    .await
                                    .unwrap();

                            client.permissions.insert(channel.id, p);
                        }

                        Payload::ServerMemberUpdate(member) => {
                            if member.id == user.id {
                                let p = Permissions::fetch(user, member.server_id.into(), None)
                                    .await
                                    .unwrap();
                                client.permissions.insert(member.server_id, p);
                            }
                        }

                        Payload::ServerMemberLeave(Empty::ServerObject { id, .. }) => {
                            if *id == user.id {
                                client.subscriptions = Subscription::Remove(vec![target_id]);
                            }
                        }

                        Payload::ServerDelete(_) => {
                            client.subscriptions = Subscription::Remove(vec![target_id]);
                        }

                        Payload::ServerCreate(server) => {
                            client.subscriptions = Subscription::Add(vec![server.id]);
                        }

                        Payload::ChannelCreate(channel) => {
                            client.subscriptions = Subscription::Add(vec![channel.id]);
                            if !channel.in_server() {
                                client.permissions.insert(
                                    channel.id,
                                    Permissions::fetch_cached(user, None, channel.into())
                                        .await
                                        .unwrap(),
                                );
                            }
                        }
                        _ => {}
                    }

                    if client.send(payload).await.is_err() {
                        break;
                    }
                }
                _ => break,
            }
        }
    });

    tokio::select!(
        _ = process => {}
    );

    log::debug!("Socket connection closed");
}
