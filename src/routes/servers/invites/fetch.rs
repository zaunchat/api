use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, invite_id)): Path<(u64, u64)>,
) -> Result<Json<Invite>> {
    Ok(Json(invite_id.invite(server_id.into()).await?))
}

pub async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Json<Vec<Invite>> {
    Json(Invite::find(|q| q.eq("server_id", server_id)).await)
}
