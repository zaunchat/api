use crate::structures::{Base, User};
use crate::utils::error::*;
use rocket::serde::json::Json;


#[get("/@me")]
fn fetch_me(user: User) -> Json<User> {
    Json(user.to_public())
}

#[get("/<user_id>")]
async fn fetch_user(user_id: u64) -> Result<Json<User>> {
    let user = User::find_one_by_id(user_id).await;

    if let Some(user) = user {
        return Ok(Json(user.to_public()));
    }

    Err(Error::UnknownUser)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_me, fetch_user]
}
