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
    Permissions::fetch(&user, None, channel_id.into())
        .await?
        .has(bits![VIEW_CHANNEL, SEND_MESSAGES])?;

    let mut msg = Message::new(channel_id, user.id);

    msg.content = data.content;

    if let Some(attachments) = data.attachments {
        msg.attachments = ormlite::types::Json(attachments);
    }

    if msg.is_empty() {
        return Err(Error::EmptyMessage);
    }

    let msg = msg.save().await?;

    Payload::MessageCreate(msg.clone()).to(channel_id).await;

    Ok(Json(msg))
}
