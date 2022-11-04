use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn open_dm(
    Extension(user): Extension<User>,
    Path(id): Path<Snowflake>,
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

    Payload::ChannelCreate(channel.clone()).to(user.id).await;

    if target.id != user.id {
        Payload::ChannelCreate(channel.clone()).to(target.id).await;
    }

    Ok(channel.into())
}
