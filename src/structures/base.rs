use crate::database::DB as db;
use rbatis::{crud::{CRUDTable, CRUD}, wrapper::Wrapper};
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Base: CRUDTable + DeserializeOwned {
    fn id(&self) -> u64;

    fn created_at(&self) -> u64 {
        todo!("Extract date from id")
    }

    async fn find<F>(query: F) -> Vec<Self>
    where
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_list_by_wrapper(query).await.unwrap()
    }

    async fn find_one<F>(query: F) -> Option<Self>
    where
        F: Send + Fn(Wrapper) -> Wrapper,
    {
        let query = query(db.new_wrapper());
        db.fetch_by_wrapper(query).await.ok()
    }

    async fn find_one_by_id(id: u64) -> Option<Self> {
        db.fetch_by_column("id", &id).await.ok()
    }

    async fn save(&self) {
        db.save(&self, &[])
            .await
            .expect("Couldn't save this target");
    }

    async fn update(&self) {
        db.update_by_column("id", &self)
            .await
            .expect("Could'nt update thus target");
    }

    async fn delete(&self) -> bool {
        db.remove_by_column::<Self, u64>("id", self.id())
            .await
            .is_ok()
    }
}
