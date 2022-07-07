use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(i64, i64)>,
) -> Result<()> {
    let channel = channel_id.channel(None).await?;

    Permissions::fetch(&user, server_id.into(), channel_id.into())
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    channel.delete(pool()).await.unwrap();

    publish(
        server_id,
        Payload::ChannelDelete((channel_id, server_id).into()),
    )
    .await;

    Ok(())
}
