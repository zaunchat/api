use crate::gateway::{
    client::Client,
    payload::{ClientPayload, Payload},
};

pub async fn run(client: &Client, _: ClientPayload) {
    client.send(Payload::Pong).await.ok();
}
