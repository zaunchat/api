use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn kick(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(Snowflake, Snowflake)>,
) -> Result<()> {
    if user.id != id {
        Permissions::fetch(&user, server_id.into(), None)
            .await?
            .has(bits![KICK_MEMBERS])?;
    }

    id.member(server_id).await?.remove().await?;

    Payload::ServerMemberLeave((id, server_id).into())
        .to(server_id)
        .await;

    Ok(())
}
