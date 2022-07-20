use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(i64, i64)>,
) -> Result<()> {
    let channel = id.channel(None).await?;

    Permissions::fetch(&user, server_id.into(), id.into())
        .await?
        .has(bits![MANAGE_CHANNELS])?;

    channel.remove().await?;

    publish(server_id, Payload::ChannelDelete((id, server_id).into())).await;

    Ok(())
}
