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
    let mut channel = id.channel(None).await?;
    let server = server_id.server(None).await?;

    Permissions::fetch_cached(&user, Some(&server), Some(&channel))
        .await?
        .has(bits![MANAGE_CHANNELS])?;

    channel.merge(data);

    let channel = channel.update_all_fields(pool()).await?;

    Payload::ChannelUpdate(channel.clone()).to(server_id).await;

    Ok(Json(channel))
}
