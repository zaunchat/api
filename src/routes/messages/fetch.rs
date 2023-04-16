use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[derive(Deserialize)]
pub struct FetchMessagesQuery {
    limit: Option<usize>,
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((channel_id, id)): Path<(Snowflake, Snowflake)>,
) -> Result<Json<Message>> {
    let msg = id.message().await?;

    if msg.channel_id != channel_id {
        return Err(Error::MissingAccess);
    }

    Permissions::fetch(&user, channel_id.into())
        .await?
        .has(bits![VIEW_CHANNEL, READ_MESSAGE_HISTORY])?;

    Ok(Json(msg))
}

pub async fn fetch_many(
    Extension(user): Extension<User>,
    Path(channel_id): Path<Snowflake>,
    Query(opt): Query<FetchMessagesQuery>,
) -> Result<Json<Vec<Message>>> {
    let channel = channel_id.channel(user.id.into()).await?;

    Permissions::fetch(&user, channel_id.into())
        .await?
        .has(bits![VIEW_CHANNEL, READ_MESSAGE_HISTORY])?;

    let messages = channel.fetch_messages(opt.limit.unwrap_or(100)).await;

    Ok(Json(messages))
}
