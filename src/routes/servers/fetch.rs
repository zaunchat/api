use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<Server>>> {
    Ok(user.fetch_servers().await?.into())
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<i64>,
) -> Result<Json<Server>> {
    Ok(id.server(user.id.into()).await?.into())
}
