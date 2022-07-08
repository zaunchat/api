use crate::config::REDIS_URI;
use fred::pool::RedisPool;
pub use fred::prelude::*;
use once_cell::sync::Lazy;
use serde::Serialize;

pub static REDIS: Lazy<RedisPool> = Lazy::new(|| {
    let config = RedisConfig::from_url((*REDIS_URI).as_str()).unwrap();
    RedisPool::new(config, 100).unwrap()
});

pub async fn connect() {
    let client = &REDIS;
    let policy = ReconnectPolicy::default();
    let _ = client.connect(Some(policy));
    let _ = client
        .wait_for_connect()
        .await
        .expect("Failed to connect to redis");
}

pub async fn pubsub() -> RedisClient {
    let config = RedisConfig::from_url((*REDIS_URI).as_str()).unwrap();
    let client = RedisClient::new(config);

    let policy = ReconnectPolicy::default();
    let _ = client.connect(Some(policy));
    let _ = client.wait_for_connect().await.unwrap();

    client
}

pub async fn publish<K: std::fmt::Display, T: Serialize>(channel: K, data: T) {
    let data = serde_json::json!(data).to_string();

    if let Err(error) = REDIS.publish::<(), _, _>(channel.to_string(), data).await {
        log::error!("Publish error: {:?}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run;
    use futures::StreamExt;

    #[test]
    fn simple() {
        run(async {
            connect().await;

            let _: () = REDIS
                .set("hello", "world", None, None, false)
                .await
                .unwrap();

            let value: String = REDIS.get("hello").await.unwrap();

            assert_eq!(value, "world");
        })
    }

    #[test]
    fn subscriber() {
        run(async {
            connect().await;

            let subscriber = pubsub().await;

            subscriber
                .subscribe("test")
                .await
                .expect("Cannot subscribe to a channel");

            let task = tokio::spawn(async move {
                if let Some((channel, message)) = subscriber.on_message().next().await {
                    log::debug!("Recv {:?} on channel {}", message, channel);
                }
            });

            publish("test", "hi").await;

            task.await.unwrap();

            let _: () = REDIS
                .set("hello", "world", None, None, false)
                .await
                .unwrap();

            let value: String = REDIS.get("hello").await.unwrap();

            assert_eq!(value, "world");
        })
    }
}
