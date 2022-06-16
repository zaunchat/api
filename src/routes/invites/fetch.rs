use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/invites/{code}",
    responses((status = 200, body = Invite), (status = 400, body = Error)),
    params(("code" = String, path))
)]
pub async fn fetch_invite(Path(code): Path<String>) -> Result<Json<Invite>> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    if let Some(invite) = invite {
        return Ok(Json(invite));
    }

    Err(Error::UnknownInvite)
}