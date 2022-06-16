use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    let permissions = Permissions::fetch(&user, server_id.into(), None).await?;

    permissions.has(Permissions::MANAGE_INVITES)?;

    invite_id.invite(server_id.into()).await?.delete().await;

    Ok(())
}
