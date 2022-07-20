use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    let channel = id.channel(user.id.into()).await?;

    if channel.owner_id != Some(user.id) {
        return Err(Error::MissingAccess);
    }

    channel.remove().await?;

    Payload::ChannelDelete(id.into()).to(id).await;

    Ok(())
}
