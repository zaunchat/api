use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;


#[utoipa::path(
    delete,
    path = "/servers/{server_id}/members/{user_id}",
    responses((status = 400, body = Error)),
    params(("server_id" = u64, path), ("user_id" = u64, path))
)]
pub async fn kick_member(
    Extension(user): Extension<User>,
    Path((server_id, member_id)): Path<(u64, u64)>,
) -> Result<()> {
    user.member_of(server_id).await?;

    if user.id != member_id {
        Permissions::fetch(&user, server_id.into(), None).await?.has(Permissions::KICK_MEMBERS)?;
    }

    member_id.member(server_id).await?.delete().await;

    Ok(())
}