use crate::config::*;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::Component)]
pub struct CreateGroupOptions {
    #[validate(length(min = 3, max = 32))]
    name: String,
}

#[utoipa::path(
    post,
    path = "/channels",
    request_body = CreateGroupOptions,
    responses((status = 200, body = Channel), (status = 400, body = Error))
)]
pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateGroupOptions>,
) -> Result<Json<Channel>> {
    let group = Channel::new_group(user.id, data.name);

    group.save().await;

    Ok(Json(group))
}