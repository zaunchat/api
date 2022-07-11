use super::client::Client;
use crate::database::redis::*;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
};
use futures::StreamExt;
use std::sync::Arc;

pub async fn upgrade(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle)
}

async fn handle(ws: WebSocket) {
    let (sender, receiver) = ws.split();
    let subscriber = pubsub().await;

    // Resubscribe on reconnect
    subscriber.manage_subscriptions();

    let client_ref = Arc::new(Client::new(sender, subscriber));

    let client = client_ref.clone();
    let mut receiver_task = tokio::spawn(async move { client.handle_incoming(receiver).await });

    let client = client_ref.clone();
    let mut sender_task = tokio::spawn(async move { client.handle_outgoing().await });

    tokio::select! {
        _ = (&mut sender_task) => receiver_task.abort(),
        _ = (&mut receiver_task) => sender_task.abort(),
    };

    log::debug!("Socket connection closed");
}
