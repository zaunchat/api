use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;




#[utoipa::path(
    delete,
    path = "/servers/{server_id}/invites/{id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn delete(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    p.has(Permissions::MANAGE_INVITES)?;

    invite_id.invite(server_id.into()).await?.delete().await;

    Ok(())
}