use crate::extractors::*;
use crate::utils::error::*;
use crate::{structures::*, utils::r#ref::Ref};

async fn fetch_one(Path(bot_id): Path<u64>) -> Result<Json<Bot>> {
    let bot = bot_id.bot().await?;
    Ok(Json(bot))
}

async fn fetch_many(Extension(user): Extension<User>) -> Json<Vec<Bot>> {
    let bots = user.fetch_bots().await;
    Json(bots)
}

async fn create() -> Result<Json<Bot>> {
    todo!()
}

async fn delete_bot(Extension(user): Extension<User>, Path(bot_id): Path<u64>) -> Result<()> {
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
        .route("/", get(fetch_many).post(create))
        .route("/:bot_id", get(fetch_one).delete(delete_bot))
}
