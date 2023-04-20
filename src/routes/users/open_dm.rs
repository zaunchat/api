use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn open_dm(
    Extension(user): Extension<User>,
    Path(id): Path<Snowflake>,
) -> Result<Json<Channel>> {
    let channel = SqlQuery::new("type = $1 AND recipients @> ARRAY[$2, $3]::BIGINT[]")
        .push(ChannelTypes::Direct)
        .push(user.id)
        .push(id)
        .find_one::<Channel>()
        .await;

    if let Ok(channel) = channel {
        return Ok(channel.into());
    }

    let target = id.user().await?;
    let channel = Channel::new_dm(user.id, target.id);

    channel.insert().await?;

    Payload::ChannelCreate(channel.clone()).to(user.id).await;

    if target.id != user.id {
        Payload::ChannelCreate(channel.clone()).to(target.id).await;
    }

    Ok(channel.into())
}
