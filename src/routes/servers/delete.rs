use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(server_id): Path<u64>) -> Result<()> {
    let server = server_id.server().await?;

    if server.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    server.delete().await;

    Ok(())
}
