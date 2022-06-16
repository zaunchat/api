use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/servers/{server_id}/roles/{id}",
    responses((status = 200, body = Role), (status = 400, body = Error)),
    params(("server_id" = u64, path), ("id" = u64, path))
)]
pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
) -> Result<Json<Role>> {
    user.member_of(server_id).await?;
    Ok(Json(role_id.role(server_id).await?))
}

#[utoipa::path(
    get,
    path = "/servers/{server_id}/roles",
    responses((status = 200, body = [Role]), (status = 400, body = Error)),
    params(("server_id" = u64, path))
)]
pub async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Role>>> {
    user.member_of(server_id).await?;

    let roles = Role::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(roles))
}