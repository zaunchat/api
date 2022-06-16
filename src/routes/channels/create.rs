use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateGroupOptions {
    #[validate(length(min = 3, max = 32))]
    name: String,
}

pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateGroupOptions>,
) -> Result<Json<Channel>> {
    let group = Channel::new_group(user.id, data.name);

    group.save().await;

    Ok(Json(group))
}
