use crate::gateway::{client::SocketClient, payload::Payload};

pub async fn run(client: &mut SocketClient, _payload: Payload) {
    client.send(Payload::Pong).await;
}
