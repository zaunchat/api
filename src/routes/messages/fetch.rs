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

    let permissions = Permissions::fetch(&user, None, channel_id.into()).await?;

    permissions.has(Permissions::VIEW_CHANNEL)?;
    permissions.has(Permissions::READ_MESSAGE_HISTORY)?;

    Ok(Json(msg))
}
