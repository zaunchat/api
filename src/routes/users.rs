use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use rocket::serde::json::Json;

#[openapi(tag = "Users")]
#[get("/@me")]
fn fetch_me(user: User) -> Result<Json<User>> {
    Ok(Json(user.to_public()))
}

#[openapi(tag = "Users")]
#[get("/<user_id>")]
async fn fetch_one(user_id: Ref) -> Result<Json<User>> {
    let user = user_id.user().await?;
    Ok(Json(user.to_public()))
}

pub fn routes() -> (Vec<rocket::Route>, rocket_okapi::okapi::openapi3::OpenApi) {
    openapi_get_routes_spec![fetch_me, fetch_one]
}
