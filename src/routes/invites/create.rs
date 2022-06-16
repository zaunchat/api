use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateInviteOptions {
    channel_id: u64,
}

pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateInviteOptions>,
) -> Result<Json<Invite>> {
    let channel = data.channel_id.channel(user.id.into()).await?;

    let p = Permissions::fetch(&user, channel.server_id, channel.id.into()).await?;

    p.has(Permissions::INVITE_OTHERS)?;

    let invite = Invite::new(user.id, channel.id, channel.server_id);

    invite.save().await;

    Ok(Json(invite))
}
