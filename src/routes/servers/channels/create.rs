// use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateServerChannelOptions {
    r#type: ChannelTypes,
    #[validate(length(min = 1, max = 32))]
    name: String,
}

pub async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<i64>,
    ValidatedJson(data): ValidatedJson<CreateServerChannelOptions>,
) -> Result<Json<Channel>> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    // let count = Channel::count(|q| q.eq("server_id", server_id)).await;

    // if count > *MAX_SERVER_CHANNELS {
    //     return Err(Error::MaximumChannels);
    // }

    let channel = match data.r#type {
        ChannelTypes::Text => Channel::new_text(data.name.clone(), server_id),
        ChannelTypes::Category => Channel::new_category(data.name.clone(), server_id),
        ChannelTypes::Voice => Channel::new_voice(data.name.clone(), server_id),
        _ => return Err(Error::MissingAccess),
    };

    let channel = channel.insert(pool()).await.unwrap();

    publish(server_id, Payload::ChannelCreate(channel.clone())).await;

    Ok(Json(channel))
}
