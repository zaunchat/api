use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[utoipa::path(
    get,
    path = "/bots/{id}",
    responses((status = 200, body = Bot), (status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn fetch_bot(Path(bot_id): Path<u64>) -> Result<Json<Bot>> {
    let bot = bot_id.bot().await?;
    Ok(Json(bot))
}

#[utoipa::path(
    get,
    path = "/bots",
    responses((status = 200, body = [Bot]), (status = 400, body = Error)),
)]
pub async fn fetch_bots(Extension(user): Extension<User>) -> Json<Vec<Bot>> {
    let bots = user.fetch_bots().await;
    Json(bots)
}

#[utoipa::path(
    post,
    path = "/bots",
    responses((status = 400, body = Error))
)]
pub async fn create_bot() -> Result<Json<Bot>> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/bots/{id}",
    responses((status = 400, body = Error)),
    params(("id" = u64, path))
)]
pub async fn delete_bot(Extension(user): Extension<User>, Path(bot_id): Path<u64>) -> Result<()> {
    let bot = bot_id.bot().await?;

    if bot.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    bot.delete().await;

    Ok(())
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_bots).post(create_bot))
        .route("/:bot_id", get(fetch_bot).delete(delete_bot))
}
