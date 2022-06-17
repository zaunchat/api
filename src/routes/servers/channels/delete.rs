use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    channel_id.channel(None).await?.delete().await;

    publish(server_id, Payload::ChannelDelete(Empty { id: channel_id })).await;

    Ok(())
}
