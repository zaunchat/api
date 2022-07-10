use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn open_dm(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> Result<Json<Channel>> {
    let channel = Channel::select()
        .filter("type = $1 AND recipients @> ARRAY[$2, $3]::BIGINT[]")
        .bind(ChannelTypes::Direct)
        .bind(user.id)
        .bind(id)
        .fetch_one(pool())
        .await;

    if let Ok(channel) = channel {
        return Ok(channel.into());
    }

    let target = id.user().await?;
    let channel = Channel::new_dm(user.id, target.id).save().await?;

    publish(user.id, Payload::ChannelCreate(channel.clone())).await;

    if target.id != user.id {
        publish(target.id, Payload::ChannelCreate(channel.clone())).await;
    }

    Ok(channel.into())
}
