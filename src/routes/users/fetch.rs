use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[utoipa::path(
    get,
    path = "/users/@me",
    responses((status = 200, body = User), (status = 400, body = Error))
)]
pub async fn fetch_me(Extension(user): Extension<User>) -> Json<User> {
    Json(user.to_public())
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses((status = 200, body = User), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_one(Path(id): Path<u64>) -> Result<Json<User>> {
    Ok(Json(id.user().await?.to_public()))
}