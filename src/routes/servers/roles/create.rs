use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateRoleOptions {
    #[validate(length(min = 1, max = 32))]
    name: String,
    color: u8,
    permissions: Permissions,
    hoist: bool,
}

pub async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateRoleOptions>,
) -> Result<Json<Role>> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(Permissions::MANAGE_ROLES)?;

    let count = Role::count(|q| q.eq("server_id", server_id)).await;

    if count > *MAX_SERVER_ROLES {
        return Err(Error::MaximumRoles);
    }

    let mut role = Role::new(data.name.clone(), server_id);

    role.permissions = data.permissions;
    role.hoist = data.hoist;
    role.color = data.color;

    role.save().await;

    publish(server_id, Payload::RoleCreate(role.clone())).await;

    Ok(Json(role))
}
