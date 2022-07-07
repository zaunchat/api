use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(server_id): Path<i64>) -> Result<()> {
    let server = server_id.server(user.id.into()).await?;

    if server.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    server.remove().await?;

    publish(server_id, Payload::ServerDelete(server_id.into())).await;

    Ok(())
}
