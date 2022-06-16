use crate::config::*;
use crate::database::DB as db;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use rbatis::crud::CRUDMut;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/servers",
    responses((status = 200, body = [Server]), (status = 400, body = Error))
)]
pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Server>> {
    Json(user.fetch_servers().await)
}

#[utoipa::path(
    get,
    path = "/servers/{id}",
    responses((status = 200, body = Server), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_one(Path(server_id): Path<u64>) -> Result<Json<Server>> {
    Ok(Json(server_id.server().await?))
}