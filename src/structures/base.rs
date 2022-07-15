use super::pool;
use ormlite::model::*;
use sqlx::{Encode, Postgres, Type};

#[async_trait]
pub trait Base: Model<Postgres> {
    async fn count(filter: &str) -> Result<u64, sqlx::Error> {
        Ok(sqlx::query(&format!(
            "SELECT COUNT(*) FROM {} WHERE {}",
            Self::table_name(),
            filter
        ))
        .execute(pool())
        .await?
        .rows_affected())
    }

    async fn save(self) -> Result<Self, ormlite::Error> {
        self.insert(pool()).await
    }

    async fn find_one<'a, Arg>(id: Arg) -> Result<Self, ormlite::Error>
    where
        Arg: 'a + Send + Encode<'a, Postgres> + Type<Postgres>,
    {
        Self::get_one(id, pool()).await
    }

    async fn remove(self) -> Result<(), ormlite::Error> {
        self.delete(pool()).await
    }
}
