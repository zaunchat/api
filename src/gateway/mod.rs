use crate::structures::User;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Extension, TypedHeader,
    },
    headers,
    response::IntoResponse,
};
use futures::stream::StreamExt;

mod client;
mod config;

pub async fn upgrade(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let config = config::SocektConfig::new(user);
    ws.on_upgrade(|ws| handle(ws, config))
}

pub async fn handle(stream: WebSocket, config: config::SocektConfig) {
    let (sender, receiver) = stream.split();
    let mut client = client::SocketClient {
        sender,
        receiver,
        config,
    };
    tokio::spawn(async move { client.handle().await });
}
