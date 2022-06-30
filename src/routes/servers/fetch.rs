use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Server>> {
    Json(user.fetch_servers().await)
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Server>> {
    Ok(Json(server_id.server(user.id.into()).await?))
}
