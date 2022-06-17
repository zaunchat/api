use crate::config::REDIS_URI;
use mobc::Pool;
pub use mobc_redis::redis::AsyncCommands;
use mobc_redis::{
    redis::{Client, RedisError, ToRedisArgs},
    RedisConnectionManager,
};
use once_cell::sync::Lazy;
use serde::Serialize;

type PooledConnection = mobc::Connection<RedisConnectionManager>;

static POOL: Lazy<Pool<RedisConnectionManager>> = Lazy::new(|| {
    let client = Client::open(REDIS_URI.to_string()).unwrap();
    let manager = RedisConnectionManager::new(client);
    Pool::builder().max_open(100).build(manager)
});

pub async fn connection() -> PooledConnection {
    POOL.get().await.unwrap()
}

pub async fn publish<
    K: ToRedisArgs + std::marker::Send + std::marker::Sync,
    T: Serialize + std::fmt::Debug,
>(
    channel: K,
    data: T,
) -> Result<(), RedisError> {
    let mut connection = connection().await;

    connection
        .publish(channel, serde_json::json!(data).to_string())
        .await?;

    Ok(())
}

pub async fn pubsub() -> redis::aio::PubSub {
    redis::Client::open(REDIS_URI.to_string())
        .unwrap()
        .get_async_connection()
        .await
        .unwrap()
        .into_pubsub()
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test() -> Result<(), redis::RedisError> {
        dotenv::dotenv().ok();

        let mut subscriber = pubsub().await;

        subscriber.subscribe("test").await?;

        let task = tokio::spawn(async move {
            println!("Spawned task");
            let mut stream = subscriber.on_message();

            while let Some(msg) = stream.next().await {
                let channel = msg.get_channel_name();
                let content: String = msg.get_payload().unwrap();

                assert!(channel == "test");
                assert!(content == "hello mom");
            }
        });

        publish("test", "hello mom").await.unwrap();

        task.await.unwrap();

        Ok(())
    }
}
