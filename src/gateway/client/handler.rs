use crate::gateway::*;
use crate::utils::{bits, Error, Permissions};
use fred::interfaces::PubsubInterface;
use futures::StreamExt;
use rmp_serde as MsgPack;
use std::sync::Arc;

pub async fn handle_incoming(client: Arc<SocketClient>, conn: Sender, receiver: &mut Receiver) {
    let mut errors = 0u8;

    while let Some(Ok(msg)) = receiver.next().await {
        let Some(payload) = client.config.decode(msg) else {
            log::debug!("Socket sent an invalid body");
            continue;
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

pub async fn handle_outgoing(client: Arc<SocketClient>) -> Result<(), Error> {
    while let Some((target_id, payload)) = client.subscriptions.on_message().next().await {
        let Ok(target_id) =  target_id.clone().try_into() else {
            log::warn!("Received non-parsable target id: {target_id:?}");
            continue;
        };

        let Some(payload) = payload.as_bytes().and_then(|buf| MsgPack::decode::from_slice(buf).ok()) else {
            log::warn!("Received non-bytes redis value: {payload:?}");
            continue;
        };

        let user_id = client.state.user_id;

        let permissions = &client.state.permissions;
        let p = permissions
            .get(&target_id)
            .map(|x| *x.value())
            .unwrap_or(Permissions::all());

        match &payload {
            Payload::MessageCreate(_) | Payload::MessageUpdate(_) | Payload::MessageDelete(_) => {
                if !p.contains(bits![VIEW_CHANNEL]) {
                    continue;
                }
            }

            Payload::ChannelDelete(Empty::Default { id }) => {
                client.subscriptions.unsubscribe(id.to_string()).await.ok();
            }

            Payload::ChannelUpdate(channel) => {
                let p = Permissions::fetch_cached(&*client.state.user.lock().await, channel.into())
                    .await?;

                permissions.insert(channel.id, p);
            }

            Payload::ChannelCreate(channel) => {
                client
                    .subscriptions
                    .subscribe(channel.id.to_string())
                    .await
                    .ok();
            }
            Payload::UserUpdate(u) => {
                // Newly friend, blocked, request
                if u.id != target_id && u.id != user_id {
                    client.subscriptions.subscribe(u.id.to_string()).await.ok();
                }
            }
            _ => {}
        }

        if client.broadcast(payload).await.is_err() {
            break; // probably the client disconnected
        }
    }

    Ok(())
}
