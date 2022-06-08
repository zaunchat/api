use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use rocket::serde::json::Json;

#[get("/<bot_id>")]
async fn fetch_one(bot_id: Ref) -> Result<Json<Bot>> {
    let bot = bot_id.bot().await?;
    Ok(Json(bot))
}

#[get("/")]
async fn fetch_many(user: User) -> Json<Vec<Bot>> {
    let bots = user.fetch_bots().await;
    Json(bots)
}

#[post("/")]
async fn create() -> Result<Json<Bot>> {
    todo!()
}

#[delete("/<bot_id>")]
async fn delete(user: User, bot_id: Ref) -> Result<()> {
    let bot = bot_id.bot().await?;

    if bot.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_one, fetch_many, create, delete]
}
