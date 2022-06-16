use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Channel>> {
    Json(user.fetch_channels().await)
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(channel_id): Path<u64>,
) -> Result<Json<Channel>> {
    let channel = channel_id.channel(user.id.into()).await?;
    Ok(Json(channel))
}
