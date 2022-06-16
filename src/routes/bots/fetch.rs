use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[utoipa::path(
    get,
    path = "/bots/{id}",
    responses((status = 200, body = Bot), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_one(Path(bot_id): Path<u64>) -> Result<Json<Bot>> {
    let bot = bot_id.bot().await?;
    Ok(Json(bot))
}

#[utoipa::path(
    get,
    path = "/bots",
    responses((status = 200, body = [Bot]), (status = 400, body = Error)),
)]
pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Bot>> {
    let bots = user.fetch_bots().await;
    Json(bots)
}