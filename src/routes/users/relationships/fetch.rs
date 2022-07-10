use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<User>>> {
    Ok(user.fetch_relations().await?.into())
}
