use crate::database::pool;
use crate::gateway::{
    client::Client,
    payload::{ClientPayload, Payload},
};
use crate::structures::*;
use crate::utils::Permissions;
use fred::interfaces::PubsubInterface;

pub async fn run(client: &Client, payload: ClientPayload) {
    if client.user.lock().await.is_some() {
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

    *client.user.lock().await = Some(user.clone());

    let mut subscriptions: Vec<i64> = vec![user.id];
    let mut permissions = client.permissions.lock().await;
    let mut channels = user.fetch_channels().await.unwrap();
    let servers = user.fetch_servers().await.unwrap();
    let users: Vec<User> = user
        .fetch_relations()
        .await
        .unwrap()
        .into_iter()
        .map(|mut u| {
            u.relationship = user.relations.0.get(&u.id).copied();
            u
        })
        .collect();

    if !servers.is_empty() {
        let server_ids: Vec<i64> = servers.iter().map(|s| s.id).collect();

        let mut other_channels = Channel::select()
            .filter("server_id = ANY($1)")
            .bind(server_ids)
            .fetch_all(pool())
            .await
            .unwrap();

        channels.append(&mut other_channels);
    }

    for user in &users {
        if user.relationship == Some(RelationshipStatus::Friend) {
            subscriptions.push(user.id);
        }
    }

    for server in &servers {
        subscriptions.push(server.id);
        permissions.insert(
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
        permissions.insert(
            channel.id,
            Permissions::fetch_cached(&user, server, channel.into())
                .await
                .unwrap(),
        );
    }

    for id in subscriptions {
        client.subscriptions.subscribe(id.to_string()).await.ok();
    }

    client
        .send(Payload::Ready {
            user,
            users,
            servers,
            channels,
        })
        .await
        .ok();
}
