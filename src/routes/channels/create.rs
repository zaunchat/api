use crate::extractors::*;
use crate::gateway::*;
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
    let group = Channel::new_group(user.id, data.name).save().await?;

    for id in group.recipients.as_ref().unwrap() {
        Payload::ChannelCreate(group.clone()).to(*id).await;
    }

    Ok(Json(group))
}
