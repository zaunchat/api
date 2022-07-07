use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((channel_id, id)): Path<(i64, i64)>,
) -> Result<()> {
    let msg = id.message().await?;
    let permissions = Permissions::fetch(&user, None, channel_id.into()).await?;

    permissions.has(Permissions::VIEW_CHANNEL)?;

    if msg.author_id != user.id {
        permissions.has(Permissions::MANAGE_MESSAGES)?;
    }

    msg.remove().await?;

    publish(channel_id, Payload::MessageDelete(id.into())).await;

    Ok(())
}
