use crate::config::*;
use crate::database::DB as db;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use rbatis::crud::CRUDMut;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    delete,
    path = "/servers/{id}",
    responses((status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn delete(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<()> {
    let server = server_id.server().await?;

    if server.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    server.delete().await;

    Ok(())
}