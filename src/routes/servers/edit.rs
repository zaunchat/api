use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;
use crate::gateway::*;


#[derive(Deserialize, Validate, OpgModel)]
pub struct EditServerOptions {
    #[validate(length(min = 1, max = 50))]
    name: String,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<EditServerOptions>
) -> Result<Json<Server>> {
    user.member_of(server_id).await?;

    Permissions::fetch(&user, server_id.into(), None).await?.has(Permissions::MANAGE_SERVER)?;

    let server = server_id.server().await?;

    if let Some(name) = data.name {
        server.name = name;
    }

    server.update().await;

    publish(server.id, Payload::ServerUpdate(server.clone())).await;

    Ok(Json(server))
}
