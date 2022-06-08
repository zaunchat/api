use crate::guards::r#ref::Ref;
use crate::structures::*;
use crate::utils::error::*;
use crate::utils::permissions::Permissions;
use rocket::serde::{json::Json};


#[get("/<server_id>/<invite_id>")]
async fn fetch_one(user: User, server_id: u64, invite_id: Ref) -> Result<Json<Invite>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    Ok(Json(invite_id.invite(server_id.into()).await?))
}

#[get("/<server_id>")]
async fn fetch_many(user: User, server_id: u64) -> Result<Json<Vec<Invite>>> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let invites = Invite::find(|q| q.eq("server_id", server_id)).await;

    Ok(Json(invites))
}

#[delete("/<server_id>/<invite_id>")]
async fn delete(user: User, server_id: u64, invite_id: Ref) -> Result<()> {
    if !user.is_in_server(server_id).await {
        return Err(Error::UnknownServer);
    }

    let p = Permissions::fetch(&user, server_id.into(), None).await?;

    if !p.contains(Permissions::MANAGE_INVITES) {
        return Err(Error::MissingPermissions);
    }

    invite_id.invite(server_id.into()).await?.delete(invite_id.0).await;

    Ok(())
}


pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_one, fetch_many, delete]
}
