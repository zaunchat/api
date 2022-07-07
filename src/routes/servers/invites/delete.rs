use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(i64, i64)>,
) -> Result<()> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_INVITES)?;

    invite_id
        .invite(server_id.into())
        .await?
        .delete(pool())
        .await?;

    Ok(())
}
