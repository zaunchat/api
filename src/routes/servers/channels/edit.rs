use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct EditServerChannelOptions {
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path((server_id, channel_id)): Path<(i64, i64)>,
    ValidatedJson(data): ValidatedJson<EditServerChannelOptions>,
) -> Result<Json<Channel>> {
    Permissions::fetch(&user, server_id.into(), channel_id.into())
        .await?
        .has(Permissions::MANAGE_CHANNELS)?;

    let mut channel = channel_id.channel(None).await?;

    if let Some(name) = data.name {
        channel.name = name.into();
    }

    let channel = channel.update_all_fields(pool()).await?;

    publish(server_id, Payload::ChannelUpdate(channel.clone())).await;

    Ok(Json(channel))
}
