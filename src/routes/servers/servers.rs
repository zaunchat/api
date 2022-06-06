use crate::structures::{Base, User. Server, Member};
use crate::utils::error::*;
use rocket::serde::{json::Json};


#[get("/")]
async fn fetch_servers(user: User) -> Json<Vec<Server>>{
    Json(user.fetch_servers().await)
}

#[get("/<server_id>")]
async fn fetch_server(server_id: u64) -> Result<Json<Server>> {
    let server = Server::find_one_by_id(server_id).await;

    if let Some(server) = server {
        return Ok(Json(server));
    }

    Err(Error::UnknownServer)
}

#[delete("/<server_id>")]
async fn delete_server(user: User, server_id: u64) -> Result<()> {
    let server = Server::find_one_by_id(server_id).await;

    if let Some(server) = server {
        if server.owner_id == user.id {
            server.delete(server.id).await;
        } else {
            let member = server.fetch_member(user.id).await;
            if let Some(member) = member {
                member.delete(member.id).await;
            }
        }

        return Ok(())
    }

    Err(Error::UnknownServer)
}


#[post("/")]
async fn create_server(user: User) {
    todo!();
}


pub fn routes() -> Vec<rocket::Route> {
    routes![fetch_servers, fetch_server, delete_server, create_server]
}