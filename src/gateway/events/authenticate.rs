use crate::gateway::{client::SocketClient, payload::Payload};
use crate::structures::*;
use crate::utils::Permissions;

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

    client.send(Payload::Authenticated).await;

    let user = user.unwrap();

    client.user = user.clone().into();
    client.subscriptions.subscribe(user.id).await.unwrap();

    let mut channels = user.fetch_channels().await;
    let servers = user.fetch_servers().await;
    let server_ids: Vec<u64> = servers.iter().map(|x| x.id).collect();

    channels.append(&mut Channel::find(|q| q.r#in("server_id", &server_ids)).await);

    for server in &servers {
        let permissions = Permissions::fetch(&user, server.id.into(), None)
            .await
            .unwrap();
        client.subscriptions.subscribe(server.id).await.unwrap();
        client.permissions.insert(server.id, permissions);
    }

    for channel in &channels {
        let permissions = Permissions::fetch(&user, channel.server_id, channel.id.into())
            .await
            .unwrap();
        client.subscriptions.subscribe(channel.id).await.unwrap();
        client.permissions.insert(channel.id, permissions);
    }

    client
        .send(Payload::Ready {
            user,
            servers,
            channels,
        })
        .await;
}
