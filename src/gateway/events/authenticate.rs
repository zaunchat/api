use crate::gateway::{
    client::{Client, Subscription},
    payload::Payload,
};
use crate::structures::*;
use crate::utils::Permissions;

pub async fn run(client: &mut Client, payload: Payload) {
    if client.user.is_some() {
        return;
    }

    let user = if let Payload::Authenticate { token } = payload {
        User::fetch_by_token(token.as_str()).await.ok()
    } else {
        None
    };

    if user.is_none() {
        return;
    }

    client.send(Payload::Authenticated).await.ok();

    let user = user.unwrap();

    client.user = Some(user.clone());

    let mut subscriptions = vec![user.id];
    let mut channels = user.fetch_channels().await;
    let servers = user.fetch_servers().await;
    let server_ids: Vec<u64> = servers.iter().map(|x| x.id).collect();

    if !server_ids.is_empty() {
        channels.append(&mut Channel::find(|q| q.r#in("server_id", &server_ids)).await);
    }

    for server in &servers {
        subscriptions.push(server.id);
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

        subscriptions.push(channel.id);
        client.permissions.insert(
            channel.id,
            Permissions::fetch_cached(&user, server, channel.into())
                .await
                .unwrap(),
        );
    }

    client.subscriptions = Subscription::Add(subscriptions);

    client
        .send(Payload::Ready {
            user,
            users: vec![], // TODO:
            servers,
            channels,
        })
        .await
        .ok();
}
