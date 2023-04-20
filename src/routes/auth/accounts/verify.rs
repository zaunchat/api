use crate::config::EMAIL_VERIFICATION;
use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VerifyQuery {
    user_id: Snowflake,
    code: Uuid,
}

pub async fn verify(Query(opts): Query<VerifyQuery>) -> Result<()> {
    if !*EMAIL_VERIFICATION {
        return Ok(());
    }

    if email::verify(opts.user_id, opts.code).await {
        let mut user = opts.user_id.user().await?;

        user.verified = true.into();
        user.update().await?;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}
