use crate::gateway::*;
use std::sync::Arc;

pub async fn run(client: Arc<SocketClient>, conn: Sender) {
    client.send(&conn, Payload::Pong).await.ok();
}
