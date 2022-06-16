use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

pub async fn verify(Path((user_id, code)): Path<(u64, String)>) -> Result<()> {
    if email::verify(user_id, &code).await {
        let mut user = user_id.user().await?;
        user.verified = true;
        user.update().await;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}
