use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    let channel = id.channel(user.id.into()).await?;

    if channel.owner_id != Some(user.id) {
        return Err(Error::MissingPermissions);
    }

    channel.remove().await?;

    publish(id, Payload::ChannelDelete(id.into())).await;

    Ok(())
}
