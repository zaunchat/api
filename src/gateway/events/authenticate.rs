use crate::gateway::{Payload, Sender, SocketClient};
use crate::structures::*;
use crate::utils::{Error, Permissions, Snowflake};
use fred::interfaces::PubsubInterface;
use std::sync::Arc;

pub async fn run(client: Arc<SocketClient>, conn: Sender) -> Result<(), Error> {
    client.send(&conn, Payload::Authenticated).await?;

    let user = client.state.user.lock().await.clone();
    let permissions = &client.state.permissions;
    let mut subscriptions: Vec<Snowflake> = vec![user.id];
    let mut channels = user.fetch_channels().await?;
    let servers = user.fetch_servers().await?;
    let users: Vec<User> = user
        .fetch_relations()
        .await?
        .into_iter()
        .map(|mut u| {
            subscriptions.push(user.id);
            u.relationship = user.relations.0.get(&u.id).copied();
            u
        })
        .collect();

    if !servers.is_empty() {
        let mut servers_channels = Channel::select()
            .filter("server_id = ANY($1)")
            .bind(servers.iter().map(|s| s.id).collect::<Vec<Snowflake>>())
            .fetch_all(pool())
            .await?;

        channels.append(&mut servers_channels);
    }

    for server in &servers {
        subscriptions.push(server.id);
        permissions.insert(
            server.id,
            Permissions::fetch_cached(&user, server.into(), None).await?,
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
            Permissions::fetch_cached(&user, server, channel.into()).await?,
        );
    }

    for id in subscriptions {
        client.subscriptions.subscribe(id.to_string()).await.ok();
    }

    client
        .send(
            &conn,
            Payload::Ready {
                user,
                users,
                servers,
                channels,
            },
        )
        .await?;

    Ok(())
}
