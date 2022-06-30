use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path((server_id, role_id)): Path<(u64, u64)>) -> Result<Json<Role>> {
    Ok(Json(role_id.role(server_id).await?))
}

pub async fn fetch_many(Path(server_id): Path<u64>) -> Json<Vec<Role>> {
    Json(Role::find(|q| q.eq("server_id", server_id)).await)
}
