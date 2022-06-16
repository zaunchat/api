use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_ROLES)?;

    role_id.role(server_id).await?.delete().await;

    Ok(())
}
