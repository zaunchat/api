use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
pub crate::gateway::*;

pub async fn delete(Extension(user): Extension<User>, Path(channel_id): Path<u64>) -> Result<()> {
    let channel = channel_id.channel(user.id.into()).await?;

    if channel.owner_id != Some(user.id) {
        return Err(Error::MissingPermissions);
    }

    channel.delete().await;

    publish(channel.id, Payload::ChannelDelete(EmptyChannel { 
        r#type: channel.r#type,
        id: channel_id,
        server_id: None,
    })).await;

    Ok(())
}
