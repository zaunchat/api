use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn kick(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(i64, i64)>,
) -> Result<()> {
    if user.id != id {
        Permissions::fetch(&user, server_id.into(), None)
            .await?
            .has(bits![KICK_MEMBERS])?;
    }

    id.member(server_id).await?.remove().await?;

    publish(
        server_id,
        Payload::ServerMemberLeave((id, server_id).into()),
    )
    .await;

    Ok(())
}
