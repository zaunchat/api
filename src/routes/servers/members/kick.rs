use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn kick(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<()> {
    if user.id != member_id {
        Permissions::fetch(&user, server_id.into(), None)
            .await?
            .has(Permissions::KICK_MEMBERS)?;
    }

    member_id.member(server_id).await?.delete().await;

    publish(
        server_id,
        Payload::ServerMemberLeave((member_id, server_id).into()),
    )
    .await;

    Ok(())
}
