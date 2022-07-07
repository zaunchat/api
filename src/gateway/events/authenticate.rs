use crate::database::pool;
use crate::gateway::{
    client::{Client, Subscription},
    payload::{ClientPayload, Payload},
};
use crate::structures::*;
use crate::utils::Permissions;

pub async fn run(client: &mut Client, payload: ClientPayload) {
    if client.user.is_some() {
        return;
    }

    let user = if let ClientPayload::Authenticate { token } = payload {
        User::fetch_by_token(token.as_str()).await
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
    let server_ids: String = servers.iter().map(|s| s.id.to_string() + ",").collect();

    if !server_ids.is_empty() {
        let mut other_channels: Vec<Channel> = sqlx::query_as(&format!(
            "SELECT * FROM channels WHERE server_id = ({})",
            server_ids
        ))
        .fetch_all(pool())
        .await
        .unwrap();
        channels.append(&mut other_channels);
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
