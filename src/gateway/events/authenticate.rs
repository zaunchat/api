use crate::gateway::{Payload, Sender, SocketClient};
use crate::utils::{Error, Permissions, Snowflake};
use fred::interfaces::PubsubInterface;
use std::sync::Arc;

pub async fn run(client: Arc<SocketClient>, conn: Sender) -> Result<(), Error> {
    client.send(&conn, Payload::Authenticated).await?;

    let user = client.state.user.lock().await.clone();
    let permissions = &client.state.permissions;
    let mut subscriptions: Vec<Snowflake> = vec![user.id];
    let channels = user.fetch_channels().await?;
    let users = user
        .fetch_relations()
        .await?
        .into_iter()
        .map(|mut u| {
            subscriptions.push(user.id);
            u.relationship = user.relations.0.get(&u.id).copied();
            u
        })
        .collect::<Vec<_>>();

    for channel in &channels {
        subscriptions.push(channel.id);
        permissions.insert(
            channel.id,
            Permissions::fetch_cached(&user, channel.into()).await?,
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
                channels,
            },
        )
        .await?;

    Ok(())
}
