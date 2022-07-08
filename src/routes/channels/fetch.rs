use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<Channel>>> {
    Ok(user.fetch_channels().await?.into())
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> Result<Json<Channel>> {
    Ok(id.channel(user.id.into()).await?.into())
}
