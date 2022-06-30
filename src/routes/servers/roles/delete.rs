use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
) -> Result<()> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_ROLES)?;

    role_id.role(server_id).await?.delete().await;

    publish(server_id, Payload::RoleDelete((role_id, server_id).into())).await;

    Ok(())
}
