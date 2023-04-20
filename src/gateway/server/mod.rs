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
    pub static ref STATE: WebSocketState = WebSocketState::new();
}

pub async fn authenticate(receiver: &mut Receiver, config: &SocketClientConfig) -> Option<User> {
    log::debug!("authenticate() called");
    let mut retries = 0;

    while let Some(Ok(msg)) = receiver.next().await {
        log::debug!("{retries} Try authenticate...");
        if retries == 3 {
            break; // nope
        }

        let Some(payload) = config.decode(msg) else {
            log::debug!("Socket sent an invalid body");
            continue;
        };

        if let ClientPayload::Authenticate { token } = payload {
            log::debug!("Provided token: {token}");
            return Some(User::fetch_by_token(&token).await.unwrap());
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
        log::info!("New socket connection");
        let (sender, mut receiver) = stream.split();
        log::debug!("Connection splitted");

        let sender = Sender::new(sender);

        log::debug!("New sender");

        let Some(user) = authenticate(&mut receiver, &config).await else {
            log::debug!("Invalid user");
            return
        };

        log::debug!("User authenticated: {}", user.username);

        let connection_id = nanoid!(6);
        let user_id = user.id;

        let client = if let Some(client) = STATE.clients.get(&user_id) {
            client
                .connections
                .insert(connection_id.clone(), sender.clone());
            Arc::clone(&client)
        } else {
            let subscriber = pubsub().await;

            // Resubscribe on reconnect
            subscriber.manage_subscriptions();

            let client = Arc::new(SocketClient::new(user, config, subscriber));

            client
                .connections
                .insert(connection_id.clone(), sender.clone());

            STATE.clients.insert(user_id, client.clone());

            let client_ref = client.clone();

            log::debug!("Spawn handle_outgoing");
            tokio::spawn(async { handle_outgoing(client_ref).await });

            client
        };

        if let Err(err) = events::authenticate::run(client.clone(), sender.clone()).await {
            log::error!("Couldn't send authenticate packets: {err}");
            return;
        }

        let client_ref = Arc::clone(&client);

        let sender_ref = sender.clone();
        log::debug!("Spawn handle_incoming");
        let receiver_task =
            tokio::spawn(
                async move { handle_incoming(client_ref, sender_ref, &mut receiver).await },
            );

        // Await client disconnection
        receiver_task.await.ok();

        log::debug!(
            "Socket disconnected (User ID: {} | Connection ID: {})",
            *user_id,
            connection_id
        );

        client.connections.remove(&connection_id);

        if client.connections.is_empty() {
            STATE.clients.remove(&user_id);
        }
    })
}
