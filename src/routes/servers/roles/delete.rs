use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(i64, i64)>,
) -> Result<()> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(bits![MANAGE_ROLES])?;

    id.role(server_id).await?.remove().await?;

    Payload::RoleDelete((id, server_id).into())
        .to(server_id)
        .await;

    Ok(())
}
