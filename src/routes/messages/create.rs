use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateMessageOptions {
    #[validate(length(min = 1, max = 2000))]
    content: String,
}

#[utoipa::path(
    post,
    request_body = CreateMessageOptions,
    path = "/messages",
    responses((status = 200, body = Message), (status = 400, body = Error))
)]
pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateMessageOptions>,
    Path(channel_id): Path<u64>
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

    msg.save().await;

    Ok(Json(msg))
}
