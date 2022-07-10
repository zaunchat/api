use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn open_dm(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> Result<Json<Channel>> {
    let channel = Channel::select()
        .filter("type = $1 AND recipients @> $2 AND recipients @> $3")
        .bind(ChannelTypes::Direct)
        .bind(user.id)
        .bind(id)
        .fetch_one(pool())
        .await;

    if let Ok(channel) = channel {
        return Ok(channel.into());
    }

    let target = id.user().await?;
    let channel = Channel::new_dm(user.id, target.id);

    Ok(channel.save().await?.into())
}
