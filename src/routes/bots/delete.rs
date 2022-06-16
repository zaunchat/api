use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn delete(Extension(user): Extension<User>, Path(bot_id): Path<u64>) -> Result<()> {
    let bot = bot_id.bot().await?;

    if bot.owner_id != user.id {
        return Err(Error::MissingAccess);
    }

    bot.delete().await;

    Ok(())
}
