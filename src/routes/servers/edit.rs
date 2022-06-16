use crate::config::*;
use crate::database::DB as db;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use rbatis::crud::CRUDMut;
use serde::Deserialize;
use validator::Validate;


#[utoipa::path(
    patch,
    path = "/servers/{id}",
    responses((status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn edit(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<()> {
    todo!()
}