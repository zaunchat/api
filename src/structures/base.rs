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

    async fn find_one<'a, Arg>(id: Arg) -> Result<Self, ormlite::Error>
    where
        Arg: 'a + Send + sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
    {
        Self::get_one(id, pool()).await
    }

    async fn remove(self) -> Result<(), ormlite::Error> {
        self.remove().await
    }
}
