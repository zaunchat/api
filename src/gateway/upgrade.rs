use super::{client::SocketClient, config::SocektConfig};
use axum::{
    extract::{ws::WebSocketUpgrade, TypedHeader},
    headers,
    response::IntoResponse,
};

pub async fn upgrade(
    ws: WebSocketUpgrade,
    _: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    ws.on_upgrade(|stream| async {
        let config = SocektConfig::new();
        let mut client = SocketClient::new(stream, config).await;
        client.handle().await
    })
}
