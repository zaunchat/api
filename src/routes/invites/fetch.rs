use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path(code): Path<String>) -> Result<Json<Invite>> {
    match Invite::find_one(code).await {
        Ok(invite) => Ok(Json(invite)),
        _ => Err(Error::UnknownInvite),
    }
}
