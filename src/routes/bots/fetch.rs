use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path(id): Path<i64>) -> Result<Json<Bot>> {
    Ok(id.bot().await?.into())
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<Bot>>> {
    Ok(user.fetch_bots().await?.into())
}
