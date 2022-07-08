use crate::extractors::*;
use crate::gateway::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(id): Path<i64>) -> Result<()> {
    let server = id.server(user.id.into()).await?;

    if server.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    server.remove().await?;

    publish(id, Payload::ServerDelete(id.into())).await;

    Ok(())
}
