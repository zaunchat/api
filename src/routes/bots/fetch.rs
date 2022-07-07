use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path(bot_id): Path<i64>) -> Result<Json<Bot>> {
    Ok(Json(bot_id.bot().await?))
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Bot>> {
    Json(user.fetch_bots().await)
}
