use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

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
