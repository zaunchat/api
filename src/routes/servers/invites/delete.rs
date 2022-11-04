use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(Snowflake, Snowflake)>,
) -> Result<()> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(bits![MANAGE_INVITES])?;

    id.invite(server_id.into()).await?.remove().await?;

    Ok(())
}
