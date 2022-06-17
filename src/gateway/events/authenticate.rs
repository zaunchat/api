use crate::gateway::{client::SocketClient, payload::Payload};
use crate::structures::*;

pub async fn run(client: &mut SocketClient, payload: Payload) {
    if client.authenticated {
        return client.close().await;
    }

    let user: Option<User>;

    if let Payload::Authenticate { token } = payload {
        user = User::fetch_by_token(token.as_str()).await.ok();
    } else {
        unreachable!()
    }

    if user.is_none() {
        return client.close().await;
    }

    let user = user.unwrap();

    client
        .send(Payload::Ready {
            user: user.clone(),
            servers: user.fetch_servers().await,
            channels: user.fetch_channels().await,
        })
        .await;
}
