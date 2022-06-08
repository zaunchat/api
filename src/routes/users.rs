use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use rocket::serde::json::Json;

#[get("/@me")]
fn fetch_me(user: User) -> Json<User> {
    Json(user.to_public())
}

#[get("/<user_id>")]
async fn fetch_one(user_id: Ref) -> Result<Json<User>> {
    let user = user_id.user().await?;
    Ok(Json(user.to_public()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_me, fetch_one]
}
