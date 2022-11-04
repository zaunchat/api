use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateInviteOptions {
    channel_id: Snowflake,
}

pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateInviteOptions>,
) -> Result<Json<Invite>> {
    let channel = data.channel_id.channel(None).await?;
    let server = if let Some(id) = channel.server_id {
        Some(id.server(None).await?)
    } else {
        None
    };

    Permissions::fetch_cached(&user, server.as_ref(), Some(&channel))
        .await?
        .has(bits![INVITE_OTHERS])?;

    let invite = Invite::new(user.id, channel.id, channel.server_id);

    Ok(Json(invite.save().await?))
}
