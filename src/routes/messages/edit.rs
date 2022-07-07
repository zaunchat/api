use crate::extractors::*;
use crate::gateway::*;
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
    Path((channel_id, id)): Path<(i64, i64)>,
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

    let msg = msg.update_all_fields(pool()).await?;

    publish(channel_id, Payload::MessageUpdate(msg.clone())).await;

    Ok(Json(msg))
}
