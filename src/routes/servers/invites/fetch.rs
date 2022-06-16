use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;



#[utoipa::path(
    get,
    path = "/servers/{server_id}/invites/{id}",
    responses((status = 200, body = Invite), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn fetch_server_invite(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<Json<Invite>> {
    user.member_of(server_id).await?;

    Ok(Json(invite_id.invite(server_id.into()).await?))
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/invites",
    responses((status = 200, body = [Invite]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_server_invites(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Invite>>> {
    user.member_of(server_id).await?;

    let invites = Invite::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(invites))
}


