use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::serde::{json::Json, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateRoleSchema<'a> {
    #[validate(length(min = 1, max = 32))]
    name: &'a str,
    color: u8,
    permissions: u64,
    hoist: bool,
}

#[derive(Deserialize, Validate)]
struct UpdateRoleSchema<'a> {
    #[validate(length(min = 1, max = 32))]
    name: Option<&'a str>,
    color: Option<u8>,
    permissions: Option<u64>,
    hoist: Option<bool>,
}

#[get("/<server_id>/<role_id>")]
async fn fetch_one(user: User, server_id: u64, role_id: Ref) -> Result<Json<Role>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    Ok(Json(role_id.role().await?))
}

#[get("/<server_id>")]
async fn fetch_many(user: User, server_id: u64) -> Result<Json<Vec<Role>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let roles = Role::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(roles))
}

#[post("/<server_id>", data = "<data>")]
async fn create(
    user: User,
    server_id: u64,
    data: Json<CreateRoleSchema<'_>>,
) -> Result<Json<Role>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    if Permissions::from_bits(data.permissions).is_none() {
        return Err(Error::MissingAccess);
    }

    let mut role = Role::new(data.name.into(), server_id);

    role.permissions = data.permissions;
    role.hoist = data.hoist;
    role.color = data.color;

    Ok(Json(role))
}

#[patch("/<server_id>/<role_id>", data = "<data>")]
async fn update(
    user: User,
    server_id: u64,
    role_id: Ref,
    data: Json<UpdateRoleSchema<'_>>,
) -> Result<Json<Role>> {
    let data = data.into_inner();

    data.validate()
        .map_err(|error| Error::InvalidBody { error })?;

    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    let mut role = role_id.role().await?;

    if let Some(name) = data.name {
        role.name = name.into();
    }

    if let Some(permissions) = data.permissions {
        if Permissions::from_bits(permissions).is_none() {
            return Err(Error::MissingAccess);
        }
        role.permissions = permissions;
    }

    if let Some(hoist) = data.hoist {
        role.hoist = hoist;
    }

    if let Some(color) = data.color {
        role.color = color;
    }

    role.update().await;

    Ok(Json(role))
}

#[delete("/<server_id>/<role_id>")]
async fn delete(user: User, server_id: u64, role_id: Ref) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    role_id.role().await?.delete(role_id.0).await;

    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_one, fetch_many, create, update, delete]
}
