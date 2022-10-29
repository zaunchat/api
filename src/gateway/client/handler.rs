use crate::utils::Permissions;
use crate::{gateway::*, utils::Ref};
use fred::interfaces::PubsubInterface;
use futures::StreamExt;
use serde_json as JSON;
use std::sync::Arc;

pub async fn handle_incoming(client: Arc<SocketClient>, conn: Sender, receiver: &mut Receiver) {
    let mut errors = 0u8;

    while let Some(Ok(msg)) = receiver.next().await {
        let payload = match client.config.decode(msg) {
            Some(p) => p,
            _ => {
                log::debug!("Socket sent an invalid body");
                continue;
            }
        };

        let res = match &payload {
            ClientPayload::Ping => events::ping::run(client.clone(), conn.clone()).await,
            // ClientPayload::Authenticate
            _ => {
                log::warn!("Unhandled event");
                Ok(())
            }
        };

        if let Err(err) = res {
            log::error!("Socket error: {err}");

            errors += 1;

            if errors == 5 {
                break;
            }
        }
    }
}

pub async fn handle_outgoing(client: Arc<SocketClient>) {
    while let Some((channel, payload)) = client.subscriptions.on_message().next().await {
        let target_id: i64 = channel.parse().unwrap();
        let user = &client.state.user.lock().await;

        let payload = JSON::from_str(&payload.as_string().unwrap()).unwrap();
        let permissions = &client.state.permissions;
        let p = permissions
            .get(&target_id)
            .map(|x| *x.value())
            .unwrap_or(Permissions::all());

        match &payload {
            Payload::MessageCreate(_) | Payload::MessageUpdate(_) | Payload::MessageDelete(_) => {
                if !p.contains(Permissions::VIEW_CHANNEL) {
                    continue;
                }
            }

            Payload::ChannelDelete(Empty::Default { id }) => {
                client.subscriptions.unsubscribe(id.to_string()).await.ok();
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
                    client
                        .subscriptions
                        .unsubscribe(target_id.to_string())
                        .await
                        .ok();
                }
            }

            Payload::ServerDelete(_) => {
                client
                    .subscriptions
                    .unsubscribe(target_id.to_string())
                    .await
                    .ok();
            }

            Payload::ServerCreate(server) => {
                client
                    .subscriptions
                    .subscribe(server.id.to_string())
                    .await
                    .ok();
            }

            Payload::ChannelCreate(channel) => {
                client
                    .subscriptions
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
                    client.subscriptions.subscribe(u.id.to_string()).await.ok();
                }
            }
            _ => {}
        }

        if client.broadcast(payload).await.is_err() {
            break;
        }
    }
}
