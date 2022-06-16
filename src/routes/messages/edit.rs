use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct EditMessageOptions {
    #[validate(length(min = 1, max = 2000))]
    content: String,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path((channel_id, id)): Path<(u64, u64)>,
    ValidatedJson(data): ValidatedJson<EditMessageOptions>,
) -> Result<Json<Message>> {
    let mut msg = id.message().await?;

    if msg.author_id != user.id || msg.channel_id != channel_id {
        return Err(Error::MissingPermissions);
    }

    Permissions::fetch(&user, None, channel_id.into())
        .await?
        .has(Permissions::VIEW_CHANNEL)?;

    msg.content = data.content.into();
    msg.update().await;

    Ok(Json(msg))
}
