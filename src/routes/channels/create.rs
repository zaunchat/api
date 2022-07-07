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
    let group = Channel::new_group(user.id, data.name)
        .insert(pool())
        .await
        .unwrap();

    for id in group.recipients.as_ref().unwrap() {
        publish(*id, Payload::ChannelCreate(group.clone())).await;
    }

    Ok(Json(group))
}
