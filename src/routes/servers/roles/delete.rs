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
        .has(Permissions::MANAGE_ROLES)?;

    id.role(server_id).await?.remove().await?;

    publish(server_id, Payload::RoleDelete((id, server_id).into())).await;

    Ok(())
}
