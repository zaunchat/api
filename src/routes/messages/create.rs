use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateMessageOptions {
    #[validate(length(min = 1, max = 2000))]
    content: String,
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

    // TODO: Add more fields
    msg.content = data.content.into();

    if msg.is_empty() {
        return Err(Error::EmptyMessage);
    }

    let msg = msg.insert(pool()).await.unwrap();

    publish(channel_id, Payload::MessageCreate(msg.clone())).await;

    Ok(Json(msg))
}
