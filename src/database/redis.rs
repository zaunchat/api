use crate::config::{REDIS_POOL_SIZE, REDIS_URI};
pub use fred::prelude::*;
use fred::{bytes_utils::Str, clients::SubscriberClient, pool::RedisPool};
use once_cell::sync::Lazy;
use rmp_serde as MsgPack;
use serde::Serialize;

pub static REDIS: Lazy<RedisPool> = Lazy::new(|| {
    let config = RedisConfig::from_url(&*REDIS_URI).expect("Invalid redis url");
    RedisPool::new(config, *REDIS_POOL_SIZE).expect("Failed initialize redis pool")
});

pub async fn connect() {
    let client = &REDIS;
    let policy = ReconnectPolicy::default();
    let _ = client.connect(Some(policy));
    client
        .wait_for_connect()
        .await
        .expect("Failed to connect to redis");
}

pub async fn pubsub() -> SubscriberClient {
    let config = RedisConfig::from_url(&*REDIS_URI).unwrap();
    let client = SubscriberClient::new(config);

    let policy = ReconnectPolicy::default();
    let _ = client.connect(Some(policy));
    client.wait_for_connect().await.unwrap();

    client
}

pub async fn publish<K: Into<Str>, V: Serialize>(channel: K, data: V) {
    let payload = match MsgPack::encode::to_vec_named(&data) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to encode payload: {e:?}");
            return;
        }
    };

    if let Err(error) = REDIS.publish::<(), _, _>(channel, payload.as_slice()).await {
        log::error!("Publish error: {error:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;
    use futures::StreamExt;

    #[test]
    fn simple() -> Result<(), RedisError> {
        run(async {
            REDIS.set("hello", "world", None, None, false).await?;

            let value: String = REDIS.get("hello").await?;

            assert_eq!(value, "world");

            Ok(())
        })
    }

    #[test]
    fn subscriber() -> Result<(), RedisError> {
        run(async {
            let subscriber = pubsub().await;

            subscriber.subscribe("test").await?;

            let task = tokio::spawn(async move {
                if let Some((channel, message)) = subscriber.on_message().next().await {
                    log::debug!("Recv {:?} on channel {}", message, channel);
                }
            });

            publish("test", "hi").await;

            task.await?;

            REDIS.set("hello", "world", None, None, false).await?;

            let value: String = REDIS.get("hello").await?;

            assert_eq!(value, "world");

            Ok(())
        })
    }
}
