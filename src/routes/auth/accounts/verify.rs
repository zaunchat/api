use crate::config::EMAIL_VERIFICATION;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VerifyQuery {
    user_id: i64,
    code: Uuid,
}

pub async fn verify(Query(opts): Query<VerifyQuery>) -> Result<()> {
    if !*EMAIL_VERIFICATION {
        return Ok(());
    }

    if email::verify(opts.user_id, opts.code).await {
        let user = opts.user_id.user().await?;
        user.update_partial().verified(true).update(pool()).await?;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}
