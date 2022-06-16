use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn fetch_one(Path(bot_id): Path<u64>) -> Result<Json<Bot>> {
    let bot = bot_id.bot().await?;
    Ok(Json(bot))
}

pub async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Bot>> {
    let bots = user.fetch_bots().await;
    Json(bots)
}
