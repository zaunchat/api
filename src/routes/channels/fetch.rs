use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/channels",
    responses((status = 200, body = [Channel]), (status = 400, body = Error))
)]
pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Channel>> {
    Json(user.fetch_channels().await)
}


#[utoipa::path(
    get,
    path = "/channels/{id}",
    responses((status = 200, body = Channel), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(channel_id): Path<u64>,
) -> Result<Json<Channel>> {
    let channel = channel_id.channel(user.id.into()).await?;
    Ok(Json(channel))
}
