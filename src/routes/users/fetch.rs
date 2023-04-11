use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_me(Extension(user): Extension<User>) -> Json<User> {
    user.into()
}

pub async fn fetch_one(
    Extension(user): Extension<User>,
    Path(id): Path<Snowflake>,
) -> Result<Json<User>> {
    if user.id == id {
        return Ok(user.into());
    }
    Ok(id.user().await?.into())
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Result<Json<Vec<User>>> {
    Ok(user.fetch_relations().await?.into())
}
