use super::pool;
use ormlite::model::*;
use sqlx::Postgres;

#[async_trait]
pub trait Base: Model<Postgres> {
    async fn count(filter: &str) -> u64 {
        sqlx::query(&format!(
            "SELECT COUNT(*) FROM {} WHERE {}",
            Self::table_name(),
            filter
        ))
        .execute(pool())
        .await
        .unwrap()
        .rows_affected()
    }
}
