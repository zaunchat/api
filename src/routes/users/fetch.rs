use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_me(Extension(user): Extension<User>) -> Json<User> {
    Json(user.to_public())
}

pub async fn fetch_one(Path(id): Path<u64>) -> Result<Json<User>> {
    Ok(Json(id.user().await?.to_public()))
}
