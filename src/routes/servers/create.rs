use crate::config::*;
use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, OpgModel)]
pub struct CreateServerOptions {
    #[validate(length(min = 1, max = 50))]
    name: String,
}

pub async fn create(
    Extension(user): Extension<User>,
    ValidatedJson(data): ValidatedJson<CreateServerOptions>,
) -> Result<Json<Server>> {
    let count = Member::count(&format!("id = {}", user.id)).await;

    if count > *MAX_SERVERS {
        return Err(Error::MaximumServers);
    }

    let server = Server::new(data.name, user.id);
    let member = Member::new(user.id, server.id);
    let category = Channel::new_category("General".into(), server.id);
    let mut chat = Channel::new_text("general".into(), server.id);

    chat.parent_id = Some(category.id);

    let mut tx = pool().begin().await.unwrap();

    let server = server.insert(&mut tx).await.unwrap();
    category.insert(&mut tx).await.unwrap();
    chat.insert(&mut tx).await.unwrap();
    member.insert(&mut tx).await.unwrap();

    tx.commit().await.unwrap();

    publish(user.id, Payload::ServerCreate(server.clone())).await;

    Ok(Json(server))
}
