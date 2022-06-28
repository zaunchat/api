use crate::extractors::*;
use crate::structures::*;
use crate::utils::*;

#[derive(Deserialize)]
pub struct VerifyQuery {
    user_id: u64,
    code: String,
}

pub async fn verify(Query(opts): Query<VerifyQuery>) -> Result<()> {
    if email::verify(opts.user_id, &opts.code).await {
        let mut user = opts.user_id.user().await?;
        user.verified = true;
        user.update().await;
        Ok(())
    } else {
        Err(Error::UnknownAccount)
    }
}
