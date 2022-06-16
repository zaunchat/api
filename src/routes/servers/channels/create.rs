use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateServerChannelOptions {
    r#type: ChannelTypes,
    #[validate(length(min = 1, max = 32))]
    name: String,
}


#[utoipa::path(
    post,
    request_body = CreateServerChannelOptions,
    path = "/servers/{server_id}/channels",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn create_server_channel(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateServerChannelOptions>,
) -> Result<Json<Channel>> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), None).await?.has(Permissions::MANAGE_CHANNELS)?;

    let count = Channel::count(|q| q.eq("server_id", server_id)).await;

    if count > *MAX_SERVER_CHANNELS {
        return Err(Error::MaximumChannels);
    }

    let channel = match data.r#type {
        ChannelTypes::Text => Ok(Json(Channel::new_text(data.name.clone(), server_id))),
        ChannelTypes::Category => Ok(Json(Channel::new_category(data.name.clone(), server_id))),
        ChannelTypes::Voice => Ok(Json(Channel::new_voice(data.name.clone(), server_id))),
        _ => Err(Error::MissingAccess),
    };

    if let Ok(channel) = &channel {
        channel.save().await;
    }

    channel
}