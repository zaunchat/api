use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    let channel = channel_id.channel(None).await?;

    Permissions::fetch(&user, server_id.into(), channel_id.into())
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    channel.delete().await;

    publish(
        server_id,
        Payload::ChannelDelete(Empty::ServerObject {
            id: channel_id,
            server_id,
        }),
    )
    .await;

    Ok(())
}
