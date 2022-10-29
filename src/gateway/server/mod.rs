mod state;

use crate::database::redis::pubsub;
use crate::gateway::events;
use crate::gateway::{
    client::handler::*, ClientPayload, Receiver, Sender, SocketClient, SocketClientConfig,
};
use crate::structures::User;
use axum::{
    extract::{ws::WebSocketUpgrade, Query},
    response::IntoResponse,
};
use futures::StreamExt;
use nanoid::nanoid;
use state::WebSocketState;
use std::sync::Arc;

lazy_static! {
    pub static ref WS: WebSocketServer = WebSocketServer::new();
}

pub struct WebSocketServer {
    pub state: WebSocketState,
}

impl Default for WebSocketServer {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn authenticate(
        &self,
        receiver: &mut Receiver,
        config: &SocketClientConfig,
    ) -> Option<User> {
        let mut retries = 0;

        while let Some(Ok(msg)) = receiver.next().await {
            if retries == 3 {
                break; // nope
            }

            let payload = match config.decode(msg) {
                Some(p) => p,
                _ => {
                    log::debug!("Socket sent an invalid body");
                    continue;
                }
            };

            if let ClientPayload::Authenticate { token } = payload {
                return User::fetch_by_token(&token).await;
            }

            retries += 1;
        }

        None
    }

    pub async fn upgrade(
        socket: WebSocketUpgrade,
        Query(config): Query<SocketClientConfig>,
    ) -> impl IntoResponse {
        socket.on_upgrade(|stream| async move {
            let (sender, mut receiver) = stream.split();
            let sender = Sender::new(sender);

            let user = match WS.authenticate(&mut receiver, &config).await {
                Some(x) => x,
                _ => return,
            };

            let connection_id = nanoid!(6);
            let user_id = user.id;

            let client = if let Some(client) = WS.state.clients.get(&user_id) {
                client
                    .connections
                    .insert(connection_id.clone(), sender.clone());
                Arc::clone(&client)
            } else {
                let subscriber = pubsub().await;

                // Resubscribe on reconnect
                subscriber.manage_subscriptions();

                let client = Arc::new(SocketClient::new(user, config, subscriber));

                WS.state.clients.insert(user_id, client.clone());

                let client_ref = client.clone();

                tokio::spawn(async { handle_outgoing(client_ref).await });

                client
            };

            if let Err(err) = events::authenticate::run(client.clone(), sender.clone()).await {
                log::error!("Couldn't send authenticate packets: {err}");
                return;
            }

            let client_ref = Arc::clone(&client);

            let sender_ref = sender.clone();
            let receiver_task = tokio::spawn(async move {
                handle_incoming(client_ref, sender_ref, &mut receiver).await
            });

            // Await client disconnection
            receiver_task.await.ok();

            log::debug!(
                "Socket disconnected (User ID: {} | Connection ID: {})",
                user_id,
                connection_id
            );

            client.connections.remove(&connection_id);

            if client.connections.is_empty() {
                WS.state.clients.remove(&user_id);
            }
        })
    }
}
