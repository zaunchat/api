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
    let channel = data.channel_id.channel(None).await?;

    Permissions::fetch(&user, channel.server_id, channel.id.into())
        .await?
        .has(Permissions::INVITE_OTHERS)?;

    let invite = Invite::new(user.id, channel.id, channel.server_id);

    Ok(Json(invite.insert(pool()).await?))
}
