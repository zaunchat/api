use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(id): Path<Snowflake>) -> Result<()> {
    let bot = id.bot().await?;

    if bot.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    bot.remove().await?;

    Ok(())
}
