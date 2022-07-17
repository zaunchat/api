use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((channel_id, id)): Path<(i64, i64)>,
) -> Result<Json<Message>> {
    let msg = id.message().await?;

    if msg.channel_id != channel_id {
        return Err(Error::MissingAccess);
    }

    Permissions::fetch(&user, None, channel_id.into())
        .await?
        .has(&[Permissions::VIEW_CHANNEL, Permissions::READ_MESSAGE_HISTORY])?;

    Ok(Json(msg))
}
