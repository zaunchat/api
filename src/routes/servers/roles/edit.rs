use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct EditRoleOptions {
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
    color: Option<i32>,
    permissions: Option<Permissions>,
    hoist: Option<bool>,
}

pub async fn edit(
    Extension(user): Extension<User>,
    Path((server_id, id)): Path<(i64, i64)>,
    ValidatedJson(data): ValidatedJson<EditRoleOptions>,
) -> Result<Json<Role>> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_ROLES)?;

    let mut role = id.role(server_id).await?;

    if let Some(name) = &data.name {
        role.name = name.clone();
    }

    if let Some(permissions) = data.permissions {
        role.permissions = permissions;
    }

    if let Some(hoist) = data.hoist {
        role.hoist = hoist;
    }

    if let Some(color) = data.color {
        role.color = color;
    }

    let role = role.update_all_fields(pool()).await?;

    publish(server_id, Payload::RoleUpdate(role.clone())).await;

    Ok(Json(role))
}
