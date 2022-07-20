use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use inter_struct::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel, StructMerge)]
#[struct_merge("crate::structures::role::Role")]
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
        .has(bits![MANAGE_ROLES])?;

    let mut role = id.role(server_id).await?;

    role.merge(data);

    let role = role.update_all_fields(pool()).await?;

    publish(server_id, Payload::RoleUpdate(role.clone())).await;

    Ok(Json(role))
}
