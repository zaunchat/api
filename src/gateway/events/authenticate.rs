use crate::gateway::{client::SocketClient, payload::Payload};
use crate::structures::*;
use crate::utils::Permissions;

pub async fn run(client: &mut SocketClient, payload: Payload) {
    if client.authenticated {
        return client.close().await;
    }

    let user = if let Payload::Authenticate { token } = payload {
        User::fetch_by_token(token.as_str()).await.ok()
    } else {
        None
    };

    if user.is_none() {
        return client.close().await;
    }

    client.send(Payload::Authenticated).await;

    let user = user.unwrap();

    client.user = Some(user.clone());
    client.subscriptions.subscribe(user.id).await.unwrap();

    let mut channels = user.fetch_channels().await;
    let servers = user.fetch_servers().await;
    let server_ids: Vec<u64> = servers.iter().map(|x| x.id).collect();

    channels.append(&mut Channel::find(|q| q.r#in("server_id", &server_ids)).await);

    for server in &servers {
        client.subscriptions.subscribe(server.id).await.unwrap();
        client.permissions.insert(
            server.id,
            Permissions::fetch_cached(&user, server.into(), None)
                .await
                .unwrap(),
        );
    }

    for channel in &channels {
        let server = if let Some(server_id) = channel.server_id {
            servers.iter().find(|s| s.id == server_id)
        } else {
            None
        };

        client.subscriptions.subscribe(channel.id).await.unwrap();
        client.permissions.insert(
            channel.id,
            Permissions::fetch_cached(&user, server, channel.into())
                .await
                .unwrap(),
        );
    }

    client
        .send(Payload::Ready {
            user,
            servers,
            channels,
        })
        .await;
}
