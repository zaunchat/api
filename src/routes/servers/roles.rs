use crate::extractors::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use crate::{structures::*, utils::r#ref::Ref};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateRoleOptions {
    #[validate(length(min = 1, max = 32))]
    name: String,
    color: u8,
    permissions: Permissions,
    hoist: bool,
}

#[derive(Deserialize, Validate)]
struct UpdateRoleOptions {
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
    color: Option<u8>,
    permissions: Option<Permissions>,
    hoist: Option<bool>,
}

async fn fetch_one(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
) -> Result<Json<Role>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    Ok(Json(role_id.role(server_id).await?))
}

async fn fetch_many(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
) -> Result<Json<Vec<Role>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let roles = Role::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(roles))
}

async fn create(
    Extension(user): Extension<User>,
    Path(server_id): Path<u64>,
    ValidatedJson(data): ValidatedJson<CreateRoleOptions>,
) -> Result<Json<Role>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    let mut role = Role::new(data.name.clone(), server_id);

    role.permissions = data.permissions;
    role.hoist = data.hoist;
    role.color = data.color;

    Ok(Json(role))
}

async fn update(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
    ValidatedJson(data): ValidatedJson<UpdateRoleOptions>,
) -> Result<Json<Role>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    let mut role = role_id.role(server_id).await?;

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

    role.update().await;

    Ok(Json(role))
}

async fn delete_role(
    Extension(user): Extension<User>,
    Path((server_id, role_id)): Path<(u64, u64)>,
) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_ROLES) {
        return Err(Error::MissingPermissions);
    }

    role_id.role(server_id).await?.delete().await;

    Ok(())
}

pub fn routes() -> axum::Router {
    use axum::{routing::*, Router};

    Router::new()
        .route("/", get(fetch_many).post(create))
        .route(
            "/:role_id",
            get(fetch_one).patch(update).delete(delete_role),
        )
}
