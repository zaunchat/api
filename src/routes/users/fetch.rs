use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_me(Extension(user): Extension<User>) -> Json<User> {
    user.into()
}

pub async fn fetch_one(Path(id): Path<i64>) -> Result<Json<User>> {
    Ok(id.user().await?.into())
}
