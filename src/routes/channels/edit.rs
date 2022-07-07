use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct EditGroupOptions {
    #[validate(length(min = 3, max = 32))]
    name: Option<String>,
}

pub async fn edit(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<EditGroupOptions>,
    Path(channel_id): Path<i64>,
) -> Result<Json<Channel>> {
    let mut group = channel_id.channel(user.id.into()).await?;

    let permissions = Permissions::fetch(&user, None, group.id.into()).await?;

    permissions.has(Permissions::MANAGE_CHANNELS)?;

    if let Some(name) = data.name {
        group.name = name.into();
    }

    let group = group.update_all_fields(pool()).await?;

    publish(group.id, Payload::ChannelUpdate(group.clone())).await;

    Ok(Json(group))
}
