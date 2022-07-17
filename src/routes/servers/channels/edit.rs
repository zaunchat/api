use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use inter_struct::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel, StructMerge)]
#[struct_merge("crate::structures::channel::Channel")]
pub struct EditServerChannelOptions {
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(i64, i64)>,
    ValidatedJson(data): ValidatedJson<EditServerChannelOptions>,
) -> Result<Json<Channel>> {
    Permissions::fetch(&user, server_id.into(), id.into())
        .await?
        .has(&[Permissions::MANAGE_CHANNELS])?;

    let mut channel = id.channel(None).await?;

    channel.merge(data);

    let channel = channel.update_all_fields(pool()).await?;

    publish(server_id, Payload::ChannelUpdate(channel.clone())).await;

    Ok(Json(channel))
}
