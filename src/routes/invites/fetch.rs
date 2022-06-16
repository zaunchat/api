use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path(code): Path<String>) -> Result<Json<Invite>> {
    let invite = Invite::find_one(|q| q.eq("code", &code)).await;

    if let Some(invite) = invite {
        return Ok(Json(invite));
    }

    Err(Error::UnknownInvite)
}
