use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateMessageOptions {
    #[validate(length(min = 1, max = 2000))]
    content: Option<String>,
    #[validate(length(max = 5))]
    attachments: Option<Vec<Attachment>>,
}

pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateMessageOptions>,
    Path(channel_id): Path<i64>,
) -> Result<Json<Message>> {
    let permissions = Permissions::fetch(&user, None, channel_id.into()).await?;

    permissions.has(Permissions::VIEW_CHANNEL)?;
    permissions.has(Permissions::SEND_MESSAGES)?;

    let mut msg = Message::new(channel_id, user.id);

    msg.content = data.content;

    if let Some(attachments) = data.attachments {
        msg.attachments = ormlite::types::Json(attachments);
    }

    if msg.is_empty() {
        return Err(Error::EmptyMessage);
    }

    let msg = msg.save().await?;

    publish(channel_id, Payload::MessageCreate(msg.clone())).await;

    Ok(Json(msg))
}
