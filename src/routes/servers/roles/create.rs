use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use inter_struct::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel, StructMerge)]
#[struct_merge("crate::structures::role::Role")]
pub struct CreateRoleOptions {
    #[validate(length(min = 1, max = 32))]
    name: String,
    color: i32,
    permissions: Permissions,
    hoist: bool,
}

pub async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<Snowflake>,
    ValidatedJson(data): ValidatedJson<CreateRoleOptions>,
) -> Result<Json<Role>> {
    Permissions::fetch(&user, server_id.into(), None)
        .await?
        .has(bits![MANAGE_ROLES])?;

    let count = Role::count(&format!("server_id = {}", *server_id)).await?;

    if count >= *MAX_SERVER_ROLES {
        return Err(Error::MaximumRoles);
    }

    let mut role = Role::new(data.name.clone(), server_id);

    role.merge(data);

    let role = role.save().await?;

    Payload::RoleCreate(role.clone()).to(server_id).await;

    Ok(Json(role))
}
