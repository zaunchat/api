use crate::gateway::{client::Client, payload::Payload};

pub async fn run(client: &mut Client, _: Payload) {
    client.send(Payload::Pong).await.ok();
}
