use super::{client::SocketClient, config::SocektConfig};
use axum::{
    extract::{ws::WebSocketUpgrade, TypedHeader},
    headers,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn upgrade(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    ws.on_upgrade(|stream| async {
        let config = SocektConfig::new();
        let client = Arc::new(Mutex::new(SocketClient::new(stream, config).await));
        tokio::join!(
            SocketClient::handle_incoming(client.clone()),
            SocketClient::handle_outcoming(client)
        );
    })
}
